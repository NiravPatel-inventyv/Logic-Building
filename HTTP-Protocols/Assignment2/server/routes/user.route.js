var express = require("express");
var route = express.Router();

const User = require("../controller/user.controller");

route.get("/users", User.getAllUsers);
route.post("/user", User.createUser);
route.get("/user/:id", User.getUserById);
route.put("/user/:id", User.updateUserById);
route.delete("/user/:id", User.deleteUserById);
module.exports = route
