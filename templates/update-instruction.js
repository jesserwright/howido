const $ = document.querySelector.bind(document);
const $$ = document.querySelectorAll.bind(document);

async function main() {
  await domLoaded();

  // Set up event listeners for modal
  // Question:
  // Can these modal toggles be async event subscriptions to be tracked and iterated through?
  // the reason being: there may be multiple events coming through (not one and done like a create/update/delete)
  // Open Modal
  // Can these be 'subscribed' to and then have the events be reacted to?
  // isn't that already what's happening?

  // BUG: update modal does not open!

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

  // Wait for the update button to be clicked
  await updateButtonClicked();

  // Get the id from end of url, convert it to int
  let id = parseInt(window.location.pathname.split("/").pop());
  let title = $("#update-instruction-input").value;

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
    // If successful, reload page
    if (response.status === 200) {
      window.location.href = `/instruction/${json.id}`;
    }
  } catch (error) {
    // Display a low-level error (like network - a fetch failure)
    displayError(error);
  }
}

function displayError(msg) {
  $("#update-instruction-input-error").textContent = msg;
}

// Make modal visable / invisible
function toggleModal() {
  // The reason for doing opacity-0 & pointer-events-none instead of 'display: none' (tailwind 'hidden')
  // is because opacity can fade-in the modal.
  $(".modal").classList.toggle("pointer-events-none");
  $(".modal").classList.toggle("opacity-0");
}

main();

function updateButtonClicked() {
  return new Promise((resolve, _reject) => {
    $("#update-instruction-button").addEventListener("click", () => {
      resolve();
    });
  });
}

function domLoaded() {
  return new Promise((resolve, _reject) => {
    window.addEventListener("DOMContentLoaded", () => {
      resolve();
    });
  });
}
