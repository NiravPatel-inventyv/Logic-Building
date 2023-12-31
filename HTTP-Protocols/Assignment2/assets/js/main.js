// Get the form and attach a submit event listener
const form = document.getElementById("RegForm");
form.addEventListener("submit", handleSubmit);

// Get the table container and create the initial table structure
const tableContainer = document.getElementById("table-container");
let table = document.createElement("table");
table.innerHTML = `
  <thead>
    <tr>
      <th class="thead"><p>FirstName</p></th>
      <th class="thead"><p>LastName</p></th>
      <th class="thead"><p>Email</p></th>
      <th class="thead" colspan="2"><p>Action</p></th>
    </tr>
  </thead>`;

// Function to fetch data and display it in the table
async function Display() {
  const response = await fetch("http://localhost:4000/users");
  const data = await response.json();

  data.forEach((item, index) => {
    // Create table rows and cells for each item
    let row = document.createElement("tr");
    let cell1 = row.insertCell(0);
    let cell2 = row.insertCell(1);
    let cell3 = row.insertCell(2);
    let cell4 = row.insertCell(3);

    // Populate cells with data and buttons
    cell1.innerHTML = `${item.firstName}`;
    cell2.innerHTML = `${item.lastName}`;
    cell3.innerHTML = `${item.email}`;
    cell4.innerHTML = `<button class="update" onclick="editData(${item.id})">Edit</button>
                        <button class="delete" onclick="deleteData(${item.id})">Delete</button>`;

    // Append the row to the table
    table.appendChild(row);
  });

  // Append the table to its container
  tableContainer.append(table);
}

// Function to handle form submission (POST request)
async function handleSubmit(e) {
  e.preventDefault();
  const data = new FormData(e.target);
  const value = Object.fromEntries(data.entries());
  const res = await fetch("http://localhost:4000/user", {
    method: "POST",
    body: JSON.stringify(value),
    headers: {
      "Content-Type": "application/json",
    },
    
  });
  const resp = await res.json();
  console.log(resp);
  form.reset();
}

// Function to edit data (GET, PUT request)
async function editData(id) {
  const response = await fetch(`http://localhost:4000/user/${id}`);
  const item = await response.json();

  // Populate input fields with data for editing
  document.getElementById("firstName").value = item.firstName;
  document.getElementById("lastName").value = item.lastName;
  document.getElementById("email").value = item.email;
  const btn = document.getElementById("btn-submit")
  btn.innerText = "update";
  btn.className = "update"
  
  // Update functionality using PUT request on form submission
  form.removeEventListener("submit", handleSubmit);
  form.addEventListener("submit", async function (e) {
    // PUT request to update data
    e.preventDefault();
    const data = new FormData(e.target);
    const value = Object.fromEntries(data.entries());

    await fetch(`http://localhost:4000/user/${id}`, {
      method: "PUT",
      body: JSON.stringify(value),
      headers: {
        "Content-Type": "application/json",
      },
    });

    form.reset();
    form.removeEventListener("submit", handleSubmit);
    form.addEventListener("submit", handleSubmit);
    
  });
}

// Function to delete data (DELETE request)
async function deleteData(id) {
  const confirmDelete = confirm("Are you sure you want to delete this item?");
  if (confirmDelete) {
    await fetch(`http://localhost:4000/user/${id}`, {
      method: "DELETE",
    });

    // After deletion, update the displayed data
    Display();
  }
}

// Initial call to fetch and display data
Display();
