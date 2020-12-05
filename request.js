let data = { title: "hi", seconds: 42 };
let url = "/step";
fetch(url, { method: "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify(data)}).then((response) => response.json()).then(console.log).catch(console.error); 