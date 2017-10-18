extern crate chrono;
extern crate clap;
#[macro_use] extern crate prettytable;
extern crate rlstats;

use std::collections::{BTreeMap, HashMap};

use rlstats::{RlStats, Player};

use chrono::{TimeZone, Utc};
use clap::{Arg, App, SubCommand};
use prettytable::format::consts::FORMAT_CLEAN;
use prettytable::row::Row;
use prettytable::Table;

static UTC_FMT: &str = "%b %e %Y";

fn print_player(cl: &RlStats, p: Player) {
    let mut info_table = table!(
        [FY -> "Display Name", p.display_name],
        [FY -> "UniqueID", p.unique_id],
        [FY -> "Platform", p.platform.name],
        [FY -> "Profile URL", p.profile_url]
    );
    info_table.set_format(*FORMAT_CLEAN);
    info_table.printstd();
    println!();

    let u = |t| Utc.timestamp(t, 0).format(UTC_FMT);
    let mut time_table = table!(
        [FY => "Requested", "Created", "Updated", "Next Update"],
        [u(p.last_requested), u(p.created_at), u(p.updated_at), u(p.next_update_at)]
    );
    time_table.set_format(*FORMAT_CLEAN);
    time_table.printstd();
    println!();

    let mut stat_table = table!(
        [FY => "Statistics"],
        ["Wins", p.stats.wins],
        ["Goals",   p.stats.goals],
        ["MVPs",    p.stats.mvps],
        ["Saves",   p.stats.saves],
        ["Shots",   p.stats.shots],
        ["Assists", p.stats.assists]
    );
    stat_table.set_format(*FORMAT_CLEAN);
    stat_table.printstd();
    println!();
    
    let pmap = cl.get_playlists()
        .expect("Could not get playlists.")
        .into_iter()
        .map(|p| (p.id.to_string(), p.name))
        .collect::<HashMap<_, _>>();

    let mut season_table = Table::init(
        p.ranked_seasons
            .iter()
            .flat_map(|(s, pls)| pls.iter().map(move |p| (s.as_str(), p)).enumerate())
            .map(|(idx, (season, (playlist, info)))| {
                row![
                    if idx == 0 { season } else { "" },
                    pmap.get(playlist).map(String::as_str).unwrap_or("Unknown"),
                    info.rank_points.unwrap_or(0),
                    info.matches_played.unwrap_or(0),
                    info.tier.unwrap_or(0),
                    info.division.unwrap_or(0)
                ]
            })
            .collect()
    );
    season_table.set_titles(row![FY => "Season", "Playlist", "Points", "Played", "Tier", "Division"]);
    season_table.set_format(*FORMAT_CLEAN);
    season_table.printstd();
}

fn main() {
    let cl = match std::env::var("RLSTATS") {
        Ok(key) => RlStats::new(key).unwrap(),
        Err(_) => {
            println!("RLSTATS environment variable not set.\nSet this to your API key.");
            std::process::exit(1);
        }
    };

    let m = App::new("rlstats")
        .version("1.0")
        .author("Scetch <lucierbrandon@gmail.com>")
        .about("Displays information from https://rocketleaguestats.com/.")
        .subcommand(SubCommand::with_name("platforms")
            .about("Display the platforms that RocketLeague supports."))
        .subcommand(SubCommand::with_name("seasons")
            .about("Display RocketLeague seasons."))
        .subcommand(SubCommand::with_name("playlists")
            .about("Display RocketLeague playlist stats."))
        .subcommand(SubCommand::with_name("tiers")
            .about("Display the current ranked tiers."))
        .subcommand(SubCommand::with_name("player")
            .about("Get a specific player on a specific platform by UniqueID.")
            .arg(Arg::with_name("id")
                .help("The UniqueID of the player.")
                .takes_value(true)
                .required(true))
            .arg(Arg::with_name("platform_id")
                .help("The platformID the player is from.")
                .takes_value(true)
                .required(true)))
        .subcommand(SubCommand::with_name("search")
            .about("Searches for a player.")
            .arg(Arg::with_name("name")
                .help("The name of the player to search for.")
                .required(true)
                .takes_value(true))
            .arg(Arg::with_name("page")
                .help("The page that should be returned.")
                .short("p")
                .long("page")
                .takes_value(true))
            .arg(Arg::with_name("select")
                .help("Display information about a specific player.")
                .short("s")
                .long("select")
                .value_name("index")))
        .subcommand(SubCommand::with_name("leaderboard")
            .about("Display ranked or stat leaderboards.")
            .subcommand(SubCommand::with_name("ranked")
                .about("Display rankings for a specific playlist id.")
                .arg(Arg::with_name("playlist_id")
                    .help("The ID of the playlist to get rankings for.")
                    .takes_value(true)
                    .required(true))
                .arg(Arg::with_name("limit")
                    .help("Limit the amount of players returned.")
                    .long("limit")
                    .short("l")
                    .takes_value(true)
                    .required(false)
                    .default_value("10"))
                .arg(Arg::with_name("select")
                    .help("Display information about a specific player.")
                    .short("s")
                    .long("select")
                    .value_name("index")))
            .subcommand(SubCommand::with_name("stat")
                .about("Get rankings based on a specific stat.")
                .arg(Arg::with_name("stat")
                    .help("The stat to get the rankings for.")
                    .takes_value(true)
                    .possible_values(&[
                        "wins", "goals", "mvps",
                        "saves", "shots", "assists"
                    ])
                    .required(true))
                .arg(Arg::with_name("limit")
                    .help("Limit the amount of players returned.")
                    .long("limit")
                    .short("l")
                    .takes_value(true)
                    .required(false)
                    .default_value("10"))
                .arg(Arg::with_name("select")
                    .help("Display information about a specific player.")
                    .short("s")
                    .long("select")
                    .value_name("index"))))
        .get_matches();

    match m.subcommand() {
        ("platforms", _) => {
            let mut t = Table::init(
                cl.get_platforms()
                    .expect("Could not get platforms.")
                    .into_iter()
                    .map(|p| row![p.id, p.name])
                    .collect()
            );
            t.set_titles(row![FY => "ID", "Platform"]);
            t.set_format(*FORMAT_CLEAN);
            t.printstd();
        }
        ("seasons", _) => {
            let mut seasons = cl.get_seasons().expect("Could not get seasons.");
            seasons.sort_by(|a, b| a.season_id.cmp(&b.season_id));

            let mut t = Table::init(
                seasons
                    .into_iter()
                    .map(|s| {
                        let started = Utc.timestamp(s.started_on, 0).format("%b %e %Y");
                        s.ended_on
                            .map(|e| row![s.season_id, started, Utc.timestamp(e, 0).format("%b %e %Y")])
                            .unwrap_or_else(|| row![FY => s.season_id, started, "Current"])
                    })
                    .collect()
            );
            t.set_titles(row![FY => "Season", "Started", "Ended"]);
            t.set_format(*FORMAT_CLEAN);
            t.printstd();
        }
        ("playlists", _) => {
            let mut platforms = cl.get_platforms().expect("Could not get platforms.");
            platforms.sort_by(|a, b| a.id.cmp(&b.id));

            // Condense the playlists.
            let (total, playlists) = cl.get_playlists()
                .expect("Could not get playlists")
                .into_iter()
                .fold((0, BTreeMap::new()), |(total, mut map), p| {
                    {
                        let pl = map.entry(p.id).or_insert((p.name, 0, BTreeMap::new()));
                        pl.1 += p.population.players;
                        pl.2.entry(p.platform_id).or_insert(p.population.players);
                    }
                    (total + p.population.players, map)
                });

            let mut t = Table::init(
                playlists
                    .into_iter()
                    .map(|(id, (name, total, pl))| {
                        let mut r = row![id, name];
                        platforms.iter()
                            .for_each(|p| {
                                r.add_cell(pl.get(&p.id)
                                    .map(|p| cell!(r -> p))
                                    .unwrap_or_else(|| cell!(r -> "N/A")))
                            });
                        r.add_cell(cell!(r -> total));
                        r
                    })
                    .collect()
            );
            
            t.add_row({
                let mut r = Row::new(vec![Default::default(); platforms.len() + 3]);
                r[platforms.len() + 2] = cell!(FYr -> total);
                r
            });

            t.set_titles({
                let mut r = row![FY => "ID", "Playlist"];
                platforms.into_iter()
                    .for_each(|p| r.add_cell(cell!(FYr -> p.name)));
                r.add_cell(cell!("Totals"));
                r
            });

            t.set_format(*FORMAT_CLEAN);
            t.printstd();
        }
        ("tiers", _) => {
            let mut t = Table::init(
                cl.get_tiers()
                    .expect("Could not get tiers.")
                    .into_iter()
                    .map(|t| row![t.id, t.name])
                    .collect()
            );
            
            t.set_titles(row![FY => "ID", "Name"]);
            t.set_format(*FORMAT_CLEAN);
            t.printstd();
        }
        ("player", Some(m)) => {
            let id = m.value_of("id").unwrap();
            let platform_id = m.value_of("platform_id")
                .and_then(|v| v.parse::<i32>().ok())
                .unwrap();

            print_player(&cl, cl.get_player(id, platform_id).expect("Could not get player."));
        }
        ("search", Some(m)) => {
            let name = m.value_of("name").unwrap();
            let page = m.value_of("page")
                .and_then(|v| v.parse::<u32>().ok())
                .unwrap_or(0);

            let mut resp = cl.search_players(name, page)
                .expect("Could not search for player.");

            if m.is_present("select") {
                let select = m.value_of("select")
                    .and_then(|v| v.parse::<usize>().ok())
                    .unwrap();
                
                // This will panic if select is out of bounds.
                // TODO: Fix.
                print_player(&cl, resp.data.swap_remove(select));
            } else {
                let mut t = Table::init(
                    resp.data.into_iter()
                        .enumerate()
                        .map(|(idx, p)| row![idx, p.display_name, p.platform.name, p.unique_id])
                        .collect()
                );
                let page = format!("Page {}", resp.page.unwrap_or(0));
                let results = format!("{} of {} results", resp.results, resp.total_results);
                t.add_row(row![FY => &page, "", "", &results]);
                t.set_titles(row![FY => "", "Display Name", "Platform", "UniqueID"]);
                t.set_format(*FORMAT_CLEAN);
                t.printstd();
            }
        }
        ("leaderboard", Some(m)) => {
            match m.subcommand() {
                ("ranked", Some(m)) => {
                    let playlist_id = m.value_of("playlist_id")
                        .and_then(|v| v.parse::<i32>().ok())
                        .unwrap();

                    let mut players = cl.get_ranked_leaderboard(playlist_id)
                            .expect("Couldn't get ranked leaderboard.");

                    if m.is_present("select") {
                        let select = m.value_of("select")
                            .and_then(|v| v.parse::<usize>().ok())
                            .unwrap();
                        
                        // This will panic if select is out of bounds.
                        // TODO: Fix.
                        print_player(&cl, players.swap_remove(select));
                    } else {
                        let limit = m.value_of("limit")
                            .and_then(|v| v.parse::<usize>().ok())
                            .unwrap(); // Has default.

                        let mut t = Table::init(
                            players.into_iter()
                                .take(limit)
                                .enumerate()
                                .map(|(idx, p)| row![idx, p.display_name, p.platform.name, p.unique_id])
                                .collect()
                        );
                        t.add_row(row![FY => "", "", "", &format!("{} of 100 results", limit)]);
                        t.set_titles(row![FY => "Rank", "Display Name", "Platform", "UniqueID"]);
                        t.set_format(*FORMAT_CLEAN);
                        t.printstd();
                    }
                }
                ("stat", Some(m)) => {
                    let stat = m.value_of("stat").unwrap(); // Required

                    let mut players = cl.get_stat_leaderboard(stat)
                        .expect("Couldn't get stat leaderboard.");

                    if m.is_present("select") {
                        let select = m.value_of("select")
                            .and_then(|v| v.parse::<usize>().ok())
                            .unwrap();
                        
                        // This will panic if select is out of bounds.
                        // TODO: Fix.
                        print_player(&cl, players.swap_remove(select));
                    } else {
                        let limit = m.value_of("limit")
                            .and_then(|v| v.parse::<usize>().ok())
                            .unwrap(); // Has default.

                        let title = match stat {
                            "wins" => "Wins",
                            "goals" => "Goals",
                            "mvps" => "MVPs",
                            "saves" => "Saves",
                            "shots" => "Shots",
                            "assists" => "Assists",
                            _ => panic!("Invalid stat."),
                        };

                        let mut t = Table::init(
                            players.into_iter()
                                .take(limit)
                                .enumerate()
                                .map(|(idx, p)| {
                                    let n = match stat {
                                        "wins" => p.stats.wins,
                                        "goals" => p.stats.goals,
                                        "mvps" => p.stats.mvps,
                                        "saves" => p.stats.saves,
                                        "shots" => p.stats.shots,
                                        "assists" => p.stats.assists,
                                        _ => panic!("Invalid stat."),
                                    };

                                    row![idx, n, p.display_name, p.platform.name, p.unique_id]
                                })
                                .collect()
                        );
                        t.add_row(row![FY => "", "", "", "", &format!("{} of 100 results", limit)]);
                        t.set_titles(row![FY => "Rank", title, "Display Name", "Platform", "UniqueID"]);
                        t.set_format(*FORMAT_CLEAN);
                        t.printstd();
                    }
                }
                _ => println!("{}", m.usage()),
            }
        }
        _ => println!("{}", m.usage()),
    }
}
