const fs = require('fs')
class Computer {
    running = false;
    x = 0
    y = 0
    z = 0
    constructor(x, y, z, running) {
        this.x = x
        this.y = y
        this.z = z
        this.running = running
    }
}
const compare = (x, y, z) => {
    if (x[0] == x[1] && y[0] === y[1] && z[0] === z[1]) {
        return true
    }
    return false
}
fs.readFile('./day17/test.txt', 'utf8', (err, data) => {
    if (err) {
        console.error(err)
        return
    }
    var array = data.trim().split("\n")
    let list = []
    array.forEach((str, x) => {
        for (y in str) {
            const element = str[y]
            list.push(new Computer(x, Number(y), 0, element === "." ? false : true))
        }
    });

    for (let cycle = 1; cycle <= 6; cycle++) {
        list = add_empty_neighbours(list, cycle)
        list = change_active_state(list, cycle)

    }
})
const change_active_state = (data, cycle) => {
    let temp_data = JSON.parse(JSON.stringify(data))
    data.forEach(target => {
        let running = 0
        var count = 0;
        data.forEach(neighbour => {
            if (neighbour.x - 1 === target.x || neighbour.x + 1 === target.x || neighbour.x === target.x) {
                if (neighbour.y - 1 === target.y || neighbour.y + 1 === target.y || neighbour.y === target.y) {
                    if (neighbour.z - 1 === target.z || neighbour.z + 1 === target.z || neighbour.z === target.z) {
                        if (!compare([neighbour.x, target.x], [neighbour.y, target.y], [neighbour.z, target.z])) {
                            count++;
                            if (neighbour.running) {
                                running += 1;
                            }
                        }
                    }
                }
            }
        })
        temp_data.forEach(el => {
            if (el.x === target.x && el.y === target.y && target.z === el.z) {
                if (target.running) {
                    if (running === 3 || running === 2) {
                        el.running = true;
                    }
                    else {
                        el.running = false
                    }
                }
                else {
                    if (running === 3) {
                        el.running = true
                    }
                }
            }
        })
    })
    var running = 0
    temp_data.forEach(el => {
        if (el.running) {
            running++;
        }
    })
    console.log(`${running} Notes running`);
    return temp_data
}
const add_empty_neighbours = (data, cycle) => {
    let add_counter = 0
    let x_map = [0 - cycle, 2 + cycle]
    let y_map = [0 - cycle, 2 + cycle]
    let z_map = [0 - cycle, 0 + cycle]

    for (let z = z_map[0]; z <= z_map[1]; z++) {
        for (let x = x_map[0]; x <= x_map[1]; x++) {
            for (let y = y_map[0]; y <= y_map[1]; y++) {
                let temp_comp = new Computer(x, y, z, false)
                let isIn = false
                data.forEach(comp => {
                    if (compare([comp.x, x], [comp.y, y], [comp.z, z])) {
                        isIn = true
                    }
                })
                if (isIn === false) {
                    data.push(temp_comp)
                    add_counter++;
                }
            }
        }
    }
    console.log(`Add in Cycle: ${cycle} => ${add_counter}`);
    return data;
}