use pathfinding::prelude::astar;
use serde::Serialize;


use crate::mapgeneration::mapgrid::MapGrid;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Pos(pub i16, pub i16);

pub fn get_path_data(map_grid: &MapGrid, start_pos: Pos, end_pos: Pos) -> Vec<Pos> {
    // let start_pos = get_pos( path_start);
    // let end_pos = get_pos( path_end);
    let path_data = get_path(map_grid, start_pos, end_pos);
    match path_data {
        Some(vec) => vec.0,
        None => vec![]
    }
}

pub fn get_path(map_grid: &MapGrid, start_pos: Pos, end_pos: Pos) -> Option<(Vec<Pos>, u32)> {
    astar(
        &start_pos,
        |p| map_grid.get_successors(p).iter().map(|s| (s.pos, s.cost)).collect::<Vec<_>>(),
        |p| ((p.0 - end_pos.0).abs() + (p.1 - end_pos.1).abs()) as u32,
        |p| *p==end_pos)
}
