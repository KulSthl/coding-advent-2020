 use std::fs;
struct Pos {
    x: usize,
    y: usize,
}
struct Place{
   pos: Pos,
   is_seat:bool,
   is_occupied: bool,
}
fn main() {
    let mut v:Vec<Vec<Place>> = vec![];
        let filedata = fs::read_to_string("./data.txt")
        .expect("Something went wrong ");
    let mut pos_x: usize  = 0;
    for line in filedata.lines() {
        let mut pos_y: usize = 0;
        let mut p_vec:Vec<Place> = line.chars().map(|s_char| {
            pos_y+=1;
            match s_char {
                'L' => Place{pos:Pos{x:pos_x,y:pos_y-1},is_seat:true,is_occupied:false},
                '.' => Place{pos:Pos{x:pos_x,y:pos_y-1},is_seat:false,is_occupied:false},
                '#' => Place{pos:Pos{x:pos_x,y:pos_y-1},is_seat:true,is_occupied:true},
                _ => panic!()
            }
        }).collect();
        v.push(p_vec);
        pos_x +=1;
    }
    print_array(&v);
    part_2(v);

}
fn part_1(mut v:Vec<Vec<Place>>) {
        for i in 0..100 {
        let mut change_list: Vec<Place> = vec![];
        for elem in &v {
            for el in elem {
                if el.is_seat {
                    let count_adjacent = get_adjacent(el,&v);
                    // let count_adjacent = adjacency_(&v,el.pos.y,el.pos.x);
                    if count_adjacent > 3 {
                        let place  = Place{pos:Pos{x:el.pos.x,y:el.pos.y},is_seat:true,is_occupied:false};
                        change_list.push(place);
                    }
                    else if count_adjacent == 0{
                        let place  = Place{pos:Pos{x:el.pos.x,y:el.pos.y},is_seat:true,is_occupied:true};
                        change_list.push(place);
                    }
                }
            }
        }
        for l_place in change_list {
            let x = l_place.pos.x;
            let y = l_place.pos.y;
            v[x][y] = l_place;
        }
        // print_array(&v);
        let mut counter_part_1: i32 = 0;
        for elem in &v {
            for el in elem {
                if el.is_occupied {
                    counter_part_1+=1;
                }
            }
        }
        println!("{}",counter_part_1);
    }
}
fn part_2(mut v:Vec<Vec<Place>>) {
    let mut prev_val = 0;
        'master: for i in 0..100 {
        let mut change_list: Vec<Place> = vec![];
        for elem in &v {
            for el in elem {
                if el.is_seat {
                    // let count_adjacent = get_adjacent(el,&v);
                    let count_adjacent = adjacency_sight_count(&v,el.pos.y,el.pos.x,el);
                    if count_adjacent > 4 {
                        let place  = Place{pos:Pos{x:el.pos.x,y:el.pos.y},is_seat:true,is_occupied:false};
                        change_list.push(place);
                    }
                    else if count_adjacent == 0{
                        let place  = Place{pos:Pos{x:el.pos.x,y:el.pos.y},is_seat:true,is_occupied:true};
                        change_list.push(place);
                    }
                }
            }
        }
        for l_place in change_list {
            let x = l_place.pos.x;
            let y = l_place.pos.y;
            v[x][y] = l_place;
        }
        // print_array(&v);
        let mut counter_part_1: i32 = 0;
        for elem in &v {
            for el in elem {
                if el.is_occupied {
                    counter_part_1+=1;
                }
            }
        }
        print_array(&v);
        println!("{}",counter_part_1);
        if prev_val == counter_part_1 {
            break 'master;
        }
        else{
            prev_val = counter_part_1;
        }
    }
}
fn get_adjacent(place : &Place,v: &[Vec<Place>]) -> usize {
    let mut count = 0;
    if place.is_seat {
        let target_x = place.pos.x as i32;
        let target_y = place.pos.y as i32;
        for elem in v {
            for l_place in elem {
                let local_x =  l_place.pos.x as i32;
                let local_y =  l_place.pos.y as i32;
                if l_place.is_seat {
                    if l_place.is_occupied {
                        if local_x >= 0 && local_x <v.len() as i32{
                            if local_y >= 0 && local_y <v[local_x as usize].len() as i32{
                                if local_y >= target_y-1 && local_y <= target_y+1{
                                    if local_x >= target_x-1 && local_x <=target_x+1{
                                        count+=1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if place.is_occupied {
        count -=1;
    }
    // println!("count: {}", count);
    count
}

fn adjacency_sight_count(field: &Vec<Vec<Place>>, __y: usize, __x: usize,place:&Place) -> usize {
    let mut count = 0;
    let x =  place.pos.x;
    let y = place.pos.y;
    // println!("Checking for {}x{}", x, y);
    // NW
    let mut idx = 1;
    'outer: for (_,elem) in field.into_iter().enumerate().filter(|(u,_)| *u < x ).rev(){
        for (_,l_place) in elem.into_iter().enumerate().filter(|(u,_) | (*u < y&&*u as i32 == (y as i32 - idx as i32))).rev() {
            if l_place.is_seat{
                if l_place.is_occupied {
                count +=1;
                }
                break 'outer;
            }
        }
        idx += 1;
    }

    // N
    let mut idx = 1;
    'outer: for (_,elem) in field.into_iter().enumerate().filter(|(u,_)| *u < x ).rev(){
        for (_,l_place) in elem.into_iter().enumerate().filter(|(u,_) | *u == y) {
            if l_place.is_seat{
                if l_place.is_occupied {
                count +=1;
                }
                break 'outer;
            }
        }
        idx += 1;
    }
    

    // NE
    let mut idx = 1;
    'outer: for (_,elem) in field.into_iter().enumerate().filter(|(u,_)| *u < x ).rev(){
        for (_,l_place) in elem.into_iter().enumerate().filter(|(u,_) | *u > y&&*u == y + idx) {
            if l_place.is_seat{
                if l_place.is_occupied {
                count +=1;
                }
                break 'outer;
            }
        }
        idx += 1;
    }

    // W
    'outer: for (_,elem) in field.into_iter().enumerate().filter(|(u,_)| *u == x ){
        for (_,l_place) in elem.into_iter().enumerate().filter(|(u,_) | *u < y).rev() {
            if l_place.is_seat{
                if l_place.is_occupied {
                count +=1;
                }
                break 'outer;
            }
        }
    }

    // E
    'outer: for (_,elem) in field.into_iter().enumerate().filter(|(u,_)| *u == x ){
        for (_,l_place) in elem.into_iter().enumerate().filter(|(u,_) | *u > y) {
            if l_place.is_seat{
                if l_place.is_occupied {
                count +=1;
                }
                break 'outer;
            }
        }
    }

    // SW
    let mut idx = 1;
    'outer: for (_,elem) in field.into_iter().enumerate().filter(|(u,_)| *u > x ){
        for (_,l_place) in elem.into_iter().enumerate().filter(|(u,_) | *u < y&&*u as i32 == (y as i32 - idx as i32) ).rev() {
            if l_place.is_seat{
                if l_place.is_occupied {
                count +=1;
                }
                break 'outer;
            }
        }
        idx += 1;
    }

    // S
    'outer: for (_,elem) in field.into_iter().enumerate().filter(|(u,_)| *u > x ){
        for (_,l_place) in elem.into_iter().enumerate().filter(|(u,_) | *u == y).rev() {
            if l_place.is_seat{
                if l_place.is_occupied {
                count +=1;
                }
                break 'outer;
            }
        }
    }

    // SE
    let mut idx = 1;
    'outer: for (_,elem) in field.into_iter().enumerate().filter(|(u,_)| *u > x ){
        for (_,l_place) in elem.into_iter().enumerate().filter(|(u,_) | *u > y&&*u == y + idx) {
            if l_place.is_seat{
                if l_place.is_occupied {
                count +=1;
                }
                break 'outer;
            }
        }
        idx += 1;
    }

    count
}
fn print_array(v:&Vec<Vec<Place>>){
    for elem in v {
        for place in elem {
            let mut ch = '.';
            if place.is_seat {
                if place.is_occupied {
                    ch = '#'
                }
                else {
                    ch = 'L'
                }
            }
            print!("{}",ch);
        }
        println!("")
    }
    println!(" ")
}