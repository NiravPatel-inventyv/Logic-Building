// perimeter and area of rectangle
//Input
let length = parseFloat(prompt("Enter length of rectangle : "))
let width = parseFloat(prompt("Enter width of rectangle : "))
//If input is valid execute the logic
if(length > 0 ){
    if(width > 0){
        let perimeter = 2 * (length + width)
        let area = length * width
        console.log(`Perimeter of rectangle is : ${perimeter.toFixed(2)} \nArea of rectangle is : ${area.toFixed(2)}`)
    }
}
//show message of invalid input
else{
    console.log("Invalid Input")
}