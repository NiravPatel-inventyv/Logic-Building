const User = require("../controller/user.controller");
function userRoutes(fastify, options, done) {
    fastify.get("/users", User.getAllUsers);
    fastify.get("/users/:id", User.getUserById);
    fastify.post("/users", User.createUser);
    fastify.put("/users/:id", User.updateUserById);
    fastify.delete("/users/:id", User.deleteUserById);
    done();
  }
  module.exports = userRoutes;
