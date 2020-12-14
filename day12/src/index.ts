/// <reference path="../typing.d.ts" />
import { data } from "./data"
interface Ship {
  direction: number;
  north: number;
  south: number;
  east: number;
  west: number;
  position?: number;
}
let ship: Ship = {
  direction: 90,
  north: 0,
  south: 0,
  west: 0,
  east: 0,
}
let ship_2: Ship = {
  direction: 0,
  north: 1,
  south: 0,
  west: 0,
  east: 10,
  position: 0,
}
let waypoint: Ship = {
  direction: 0,
  north: 0,
  south: 0,
  west: 0,
  east: 0,
  position: 0,
}

const handle = (action: string, num: number) => {

  if (action === "R") {
    ship.direction = (ship.direction + num) % 360;
    const rotation = (num) % 360;
    handleRotation(rotation);
    return;
  }
  else if (action === "L") {
    ship.direction = (ship.direction + 360 - num) % 360;
    const rotation = (360 - num) % 360;
    handleRotation(rotation);
    return;
  }
  else if (action === "F") {
    calc_waypoint(num);
    if (ship.direction === 0) {
      ship.north += num;
      return;
    }
    if (ship.direction === 90) {
      ship.east += num;
      return;
    }
    if (ship.direction === 180) {
      ship.south += num;
      return;
    }
    if (ship.direction === 270) {
      ship.west += num;
      return;
    }
  }
  else {
    if (action === "N") {
      ship.north += num;
      ship_2.north += num;
      return;
    }
    if (action === "E") {
      ship.east += num;
      ship_2.east += num;
      return;
    }
    if (action === "S") {
      ship.south += num;
      ship_2.south += num;
      return;
    }
    if (action === "W") {
      ship.west += num;
      ship_2.west += num;
      return;
    }
  }
}
const calc = (temp_ship: Ship) => {
  let e_w = (temp_ship.east - temp_ship.west);
  let n_s = (temp_ship.north - temp_ship.south);
  n_s = n_s > 0 ? n_s : n_s * -1;
  e_w = e_w > 0 ? e_w : e_w * -1;
  return n_s + e_w
}
const calc_waypoint = (factor: number) => {
  waypoint.east += ship_2.east * factor
  waypoint.west += ship_2.west * factor
  waypoint.north += ship_2.north * factor
  waypoint.south += ship_2.south * factor
}
const handleRotation = (rotation: number) => {
  console.log(rotation);

  const temp = JSON.parse(JSON.stringify(ship_2)) as Ship;
  if (rotation === 270) {
    ship_2.north = temp.east
    ship_2.east = temp.south
    ship_2.south = temp.west
    ship_2.west = temp.north
  }
  if (rotation === 180) {
    ship_2.north = temp.south
    ship_2.east = temp.west
    ship_2.south = temp.north
    ship_2.west = temp.east
  }
  if (rotation === 90) {
    ship_2.north = temp.west
    ship_2.east = temp.north
    ship_2.south = temp.east
    ship_2.west = temp.south
  }

}
const arr = data.trim().split("\n")
const formatted = arr.map(line => {
  const action = line[0]
  const num = line.substring(1)
  handle(action, Number(num));
  console.log(line)
  console.log(waypoint)
  // console.log(ship_2)
})

// console.log(ship);
// console.log(calc(ship));
// console.log(ship_2)
// console.log(waypoint)
console.log(calc(waypoint))

