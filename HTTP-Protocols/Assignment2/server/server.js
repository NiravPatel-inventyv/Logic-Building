//importing required modules
const express = require('express')
const cors = require('cors');

const app = express()
const PORT = 4000

// allowing cross-origin requests
app.use(express.json())
app.use(cors())


const User = require('./routes/user.route')

app.use('/',User)
//listening server
app.listen(PORT, () => {
    console.log(`Server started on port ${PORT}...`);
  });