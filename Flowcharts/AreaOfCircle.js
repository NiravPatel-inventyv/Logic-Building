//Find area of circle

//func to calculate area
function Area(r){
    return Math.PI * r * r;
}
//Input
let radius = parseFloat(prompt("Enter the radius of the circle: "));
//If input is valid execute the logic
if(radius > 0){
    let area = Area(radius)
    console.log(`Area of circle is : ${area.toFixed(3)}`)
}
//show message of invalid input
else{
    console.log("Invalid input")
}