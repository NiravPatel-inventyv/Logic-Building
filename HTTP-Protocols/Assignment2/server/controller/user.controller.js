const fs = require('fs');
const path = require('path');

// Define the path to the JSON file
const filePath = path.join(__dirname, '../data.json');

// Function to read data from the JSON file
const readDataFromFile = () => {
  try {
    const data = fs.readFileSync(filePath, 'utf8');
    return JSON.parse(data);
  } catch (error) {
    // Return an empty array if there's an error or if the file doesn't exist
    return { users: [] };
  }
};

// Function to write data to the JSON file
const writeDataToFile = (data) => {
  try {
    fs.writeFileSync(filePath, JSON.stringify(data, null, 2), 'utf8');
  } catch (error) {
    console.error('Error writing to file:', error);
  }
};

// Function to create a new user
const createUser = (req, res) => {
  const { firstName, lastName, email } = req.body;

  // Read existing data from the file
  const allUsers = readDataFromFile();

  // Generate a new ID for the user
  const newUserId = allUsers.users.length + 1;

  // Create a new user object
  const newUser = {
    id: newUserId,
    firstName,
    lastName,
    email,
  };

  // Add the new user to the existing data and write back to the file
  allUsers.users.push(newUser);
  writeDataToFile(allUsers);

  // Return the newly created user in the response
  res.status(201).json(newUser);
};

// Function to get all users
const getAllUsers = (req, res) => {
  // Get all users from the file and send them in the response
  const users = readDataFromFile().users;
  res.status(200).json(users);
};

// Function to get a user by ID
const getUserById = (req, res) => {
  const userId = req.params.id;
  const user = readDataFromFile().users.find((u) => u.id === parseInt(userId));

  if (!user) {
    return res.status(404).json({ message: 'User not found' });
  }

  // Return the found user in the response
  res.status(200).json(user);
};

// Function to update a user by ID
const updateUserById = (req, res) => {
  const userId = req.params.id;
  const { firstName, lastName, email } = req.body;

  let allUsers = readDataFromFile();
  const userIndex = allUsers.users.findIndex((user) => user.id === parseInt(userId));

  if (userIndex !== -1) {
    // Update the user's data and write back to the file
    allUsers.users[userIndex] = {
      ...allUsers.users[userIndex],
      firstName,
      lastName,
      email,
      id: parseInt(userId),
    };

    writeDataToFile(allUsers);

    // Return the updated user in the response
    res.status(200).json(allUsers.users[userIndex]);
  } else {
    res.status(404).json({ message: 'User not found' });
  }
};

// Function to delete a user by ID
const deleteUserById = (req, res) => {
  const userId = req.params.id;

  let allUsers = readDataFromFile();
  const updatedUsers = allUsers.users.filter((user) => user.id !== parseInt(userId));

  if (updatedUsers.length !== allUsers.users.length) {
    // Remove the user from the data and write back to the file
    allUsers.users = updatedUsers;
    writeDataToFile(allUsers);

    // Respond with a success message
    res.status(200).json({ message: 'User deleted successfully' });
  } else {
    res.status(404).json({ message: 'User not found' });
  }
};

module.exports = {
  createUser,
  getAllUsers,
  getUserById,
  updateUserById,
  deleteUserById,
};
