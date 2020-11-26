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

    // TODO: set the input back to the original value when the modal is closed

    $(".modal").classList.toggle("pointer-events-none");
    $(".modal").classList.toggle("opacity-0");
  }

  $("#create-instruction-button").addEventListener("click", createInstruction);

  function displayError(msg) {
    $("#create-instruction-input-error").textContent = msg;
  }

  async function createInstruction() {
    let title = $("#create-instruction-input").value;
    try {
      const response = await fetch("/instruction", {
        method: "post",
        headers: {
          Accept: "application/json",
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ title }),
      });
      const json = await response.json();
      console.log(json);

      // Check for validation error
      if (response.status === 422) {
        $("#create-instruction-input").value = json.input;
        // Display validation error
        displayError(json.msg);
      }
      // If successful, redirect to route
      if (response.status === 200) {
        // Clear the input field
        $("#create-instruction-input").value = "";
        window.location.href = `/instruction/${json.id}`;
      }
    } catch (error) {
      // Display a low-level error (like network - a fetch failure)
      displayError(error);
    }
  }
});
