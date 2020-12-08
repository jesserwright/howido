// I want this to be async-friendly. With a main() method that runs it??
// Really, async DOM seems to be the way to go.

const wait = ms => new Promise((resolve) => setTimeout(resolve, ms));

window.addEventListener("load", () => {
  // Shorthand selectors
  const $ = document.querySelector.bind(document);
  const $$ = document.querySelectorAll.bind(document);

  // Element refrences
  const inputEl = $("#create-instruction-input");
  const modalEl = $(".modal");

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

  function toggleModal() {
    // The reason for doing opacity-0 & pointer-events-none instead of 'display: none' (tailwind 'hidden')
    // is because opacity can fade-in the modal.

    const modalClassList = modalEl.classList;
    const open = !modalClassList.contains("opacity-0");

    if (!open) {
      // closed => open
      inputEl.focus();
    } else {
      // open => closed
      // timeout, to let the fade out transition happen before clearing field
      setTimeout(() => {
        inputEl.value = "";
      }, 150);
    }

    modalClassList.toggle("pointer-events-none");
    modalClassList.toggle("opacity-0");
  }

  $("#create-instruction-button").addEventListener("click", createInstruction);

  function displayError(msg) {
    $("#create-instruction-input-error").textContent = msg;
  }

  async function createInstruction() {
    let title = inputEl.value;
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
        inputEl.value = json.input;
        // Display validation error
        displayError(json.msg);
      }
      // If successful, redirect to route
      if (response.status === 200) {
        // Clear the input field. (Is this saved in browser state?! WHY?)
        inputEl.value = "";
        window.location.href = `/instruction/${json.id}`;
      }
    } catch (error) {
      // Display a low-level error (like network - a fetch failure)
      displayError(error);
    }
  }
});
