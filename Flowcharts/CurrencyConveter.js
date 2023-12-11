//Convert the currency unit
//Input
let Type = prompt("Enter type in which you want to convert (rs/ps) : ")
//If type is paise 
if(Type == "ps" ){
    let amount = parseFloat(prompt("Enter amount :"))
    let paise  = amount * 100
    console.log(`The amount is ${paise} paise.`)
}else{
    //if type is rs
    if(Type == "rs" ){
        let amount = parseFloat(prompt("Enter amount :"))
        let rupee = amount / 100
        console.log(`The amount is ${rupee.toFixed(2)} rs.`)
    }
    // any other input
else{
    console.log("Invalid input")
}
} 