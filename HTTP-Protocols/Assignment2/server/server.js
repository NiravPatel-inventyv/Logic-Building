const fs = require('fs');
const path = require('path');
const privateKey = fs.readFileSync(path.join(__dirname, 'server.key'), 'utf8');
const certf = fs.readFileSync(path.join(__dirname, 'certificate.crt'));
const fastifyCors = require("@fastify/cors");
const fastify = require('fastify')({
  http2: true,
  https: {
    key: privateKey,
    cert: certf
  }
})
const userRoutes = require('./routes/user.route');

fastify.register(fastifyCors,{
  origin: '*', 
  methods: 'GET,POST,PUT,DELETE', 
  allowedHeaders: 'Content-Type,Authorization',
});
fastify.register(userRoutes);
fastify.get('/', function (request, reply) {
  reply.code(200).send({ hello: 'world' })
})
fastify.get('/home', function (request, res) {
  res.code(200).send({ message : "See you soon" })
})

fastify.listen(3000,()=>{
  console.log('Server running on http://localhost:3000')
})