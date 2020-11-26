// CHANGE THIS TO REFLECT THE UPDATE INSTEAD OF CREATE
// Get id from URI
// Make the classes different.

// Perhaps note what is the same and different, just a little. (what DOM elements, and classes are
// the same / different across usages)

let $ = document.querySelector.bind(document);
let $$ = document.querySelectorAll.bind(document);
window.addEventListener("load", () => {
  // Open Modal
  $$(".modal-open").forEach((el) => {
    el.addEventListener("click", (evt) => {
      evt.preventDefault();
      toggleModal();
    });
  });
  // Close Modal
  $$(".modal-close").forEach((el) => {
    el.addEventListener("click", toggleModal);
  });
  // Make modal visable / invisible
  function toggleModal() {
    // The reason for doing opacity-0 & pointer-events-none instead of 'display: none' (tailwind 'hidden')
    // is because opacity can fade-in the modal.
    $(".modal").classList.toggle("pointer-events-none");
    $(".modal").classList.toggle("opacity-0");
  }

  $("#update-instruction-button").addEventListener("click", updateInstruction);

  function displayError(msg) {
    $("#update-instruction-input-error").textContent = msg;
  }

  async function updateInstruction() {
    let title = $("#update-instruction-input").value;
    // Get the id from end of url, convert it to int
    let id = parseInt(window.location.pathname.split("/").pop(), 10);
    try {
      const response = await fetch("/instruction", {
        method: "put",
        headers: {
          Accept: "application/json",
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ title, id }),
      });
      const json = await response.json();
      console.log(json);

      // Check for validation error
      if (response.status === 422) {
        $("#update-instruction-input").value = json.input;
        // Display validation error
        displayError(json.msg);
      }
      // If successful, redirect to route
      if (response.status === 200) {
        window.location.href = `/instruction/${json.id}`;
      }
    } catch (error) {
      // Display a low-level error (like network - a fetch failure)
      displayError(error);
    }
  }
});
