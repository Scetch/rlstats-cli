extern crate chrono;
extern crate clap;
#[macro_use] extern crate prettytable;
extern crate rlstats;

use std::collections::{BTreeMap, HashMap};

use rlstats::{RlStats, Player};

use chrono::{TimeZone, Utc};
use clap::{Arg, App, SubCommand};
use prettytable::format::consts::FORMAT_CLEAN;
use prettytable::cell::Cell;
use prettytable::row::Row;
use prettytable::Table;

fn print_player(cl: &RlStats, p: Player) {
    // Grab the playlist names since we'll need them later.
    let pmap = cl.get_playlists()
        .expect("Could not get playlists.")
        .into_iter()
        .map(|p| (p.id, p.name))
        .collect::<HashMap<_, _>>();

    let mut info_table = table!(
        [FY -> "Display Name", p.display_name],
        [FY -> "UniqueID", p.unique_id],
        [FY -> "Platform", p.platform.name],
        [FY -> "Profile URL", p.profile_url]
    );
    info_table.set_format(*FORMAT_CLEAN);
    info_table.printstd();

    println!();

    let requested = Utc.timestamp(p.last_requested, 0).format("%b %e %Y");
    let created = Utc.timestamp(p.created_at, 0).format("%b %e %Y");
    let updated = Utc.timestamp(p.updated_at, 0).format("%b %e %Y");
    let next = Utc.timestamp(p.next_update_at, 0).format("%b %e %Y");

    let mut time_table = table!(
        [FY => "Requested", "Created", "Updated", "Next Update"],
        [requested, created, updated, next]
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

    let mut season_table = table!(
        [FY => "Ranked"],
        [Fy => "Season", "Playlist", "Points", "Played", "Tier", "Division"]
    );
    season_table.set_format(*FORMAT_CLEAN);

    for (season, playlists) in p.ranked_seasons {
        for (idx, (playlist, info)) in playlists.iter().enumerate() {
            let playlist = playlist.parse::<i32>().unwrap();

            season_table.add_row(row![
                if idx == 0 { cell!(season) } else { cell!("") },
                pmap.get(&playlist).unwrap_or(&"Unknown".to_owned()),
                info.rank_points.unwrap_or(0),
                info.matches_played.unwrap_or(0),
                info.tier.unwrap_or(0),
                info.division.unwrap_or(0)
            ]);
        }
    }
    
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
            let mut t = table!([FY => "ID", "Platform"]);
            t.set_format(*FORMAT_CLEAN);

            cl.get_platforms()
                .expect("Could not get platforms.")
                .into_iter()
                .for_each(|p| { t.add_row(row![p.id, p.name]); });

            t.printstd();
        }
        ("seasons", _) => {
            let mut seasons = cl.get_seasons()
                .expect("Could not get seasons.");
            
            seasons.sort_by(|a, b| a.season_id.cmp(&b.season_id));

            let mut t = table!([FY => "Season", "Started", "Ended"]);
            t.set_format(*FORMAT_CLEAN);

            for s in seasons {
                let started = Utc.timestamp(s.started_on, 0).format("%b %e %Y");
                let ended = s.ended_on.map(|e| Utc.timestamp(e, 0).format("%b %e %Y"));

                t.add_row(ended
                    .map(|e| row![s.season_id, started, e])
                    .unwrap_or_else(|| row![Fy => s.season_id, started, "Current"]));
            }

            t.printstd();
        }
        ("playlists", _) => {
            let mut platforms = cl.get_platforms()
                .expect("Could not get platforms.");
            
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
            
            // Build the table.
            let mut t = Table::new();
            t.set_format(*FORMAT_CLEAN);

            // Build categories.
            {
                let r = t.add_row(row![FY => "ID", "Playlist"]);
                platforms.iter().for_each(|p| r.add_cell(cell!(FYr -> p.name)));
                r.add_cell(cell!(FYr -> "Total"));
            }

            // Build rows.
            playlists.into_iter()
                .for_each(|(id, (name, total, pl))| {
                    let r = t.add_row(row![id, name]);

                    platforms.iter()
                        .map(|p| pl.get(&p.id)
                            .map(|p| cell!(r -> p))
                            .unwrap_or_else(|| cell!(r -> "N/A")))
                        .for_each(|c| r.add_cell(c));

                    r.add_cell(cell!(r -> total));
                });
            

            // Build final row.
            t.add_row(Row::new(vec![Cell::default(); platforms.len() + 3]))
                .set_cell(cell!(FYr -> total), platforms.len() + 2).unwrap();

            t.printstd();
        }
        ("tiers", _) => {
            let mut t = table!([FY => "ID", "Name"]);
            t.set_format(*FORMAT_CLEAN);

            cl.get_tiers()
                .expect("Could not get tiers.")
                .into_iter()
                .for_each(|tier| { t.add_row(row![tier.id, tier.name]); });

            t.printstd();
        }
        ("player", Some(m)) => {
            let id = m.value_of("id").unwrap();
            let platform_id = m.value_of("platform_id")
                .and_then(|v| v.parse::<i32>().ok())
                .unwrap();

            let player = cl.get_player(id, platform_id)
                .expect("Could not get player.");
            
            print_player(&cl, player);
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
                let mut t = table!([FY => "", "Display Name", "Platform", "UniqueID"]);
                t.set_format(*FORMAT_CLEAN);
                
                resp.data
                    .iter()
                    .enumerate()
                    .for_each(|(idx, p)| {
                        t.add_row(row![idx, p.display_name, p.platform.name, p.unique_id]);
                    });

                t.add_row(row![
                    FY => &format!("Page {}", resp.page.unwrap_or(0)),
                    "",
                    "",
                    &format!("{} of {} results", resp.results, resp.total_results)
                ]);

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

                        let mut t = table!([FY => "Rank", "Display Name", "Platform", "UniqueID"]);
                        t.set_format(*FORMAT_CLEAN);

                        // Builds rows.
                        players.into_iter()
                            .take(limit)
                            .enumerate()
                            .for_each(|(idx, p)| {
                                t.add_row(row![idx, p.display_name, p.platform.name, p.unique_id]);
                            });

                        t.add_row(row![FY => "", "", "", &format!("{} of 100 results", limit)]);

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

                        let mut t = table!([FY => "Rank", title, "Display Name", "Platform", "UniqueID"]);
                        t.set_format(*FORMAT_CLEAN);

                        players.into_iter()
                            .take(limit)
                            .enumerate()
                            .for_each(|(idx, p)| {
                                let n = match stat {
                                    "wins" => p.stats.wins,
                                    "goals" => p.stats.goals,
                                    "mvps" => p.stats.mvps,
                                    "saves" => p.stats.saves,
                                    "shots" => p.stats.shots,
                                    "assists" => p.stats.assists,
                                    _ => panic!("Invalid stat."),
                                };

                                t.add_row(row![idx, n, p.display_name, p.platform.name, p.unique_id]);
                            });


                        t.add_row(row![FY => "", "", "", "", &format!("{} of 100 results", limit)]);
                        t.printstd();
                    }
                }
                _ => println!("{}", m.usage()),
            }
        }
        _ => println!("{}", m.usage()),
    }
}
