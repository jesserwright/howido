let data = { id: 1 };
let url = "/step";

fetch(url, {
  method: "DELETE",
  headers: { "Content-Type": "application/json" },
  body: JSON.stringify(data),
})
  .then((response) => response.json())
  .then(console.log)
  .catch(console.error);
