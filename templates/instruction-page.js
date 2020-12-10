// Do some styling

// Alias the verbose DOM API
const $ = document.querySelector.bind(document);
const $$ = document.querySelectorAll.bind(document);

// `main` binds elements and functions with event listeners
function main() {
  // Toggle modal
  $$(".toggle-modal").forEach((el) => {
    el.addEventListener("click", (evt) => {
      evt.preventDefault();
      toggleModal();
    });
  });
  // Update Instruction
  $("#update-instruction-button").addEventListener("click", updateInstruction);
  // Create Step
  $("#create-step-button").addEventListener("click", createStep);
  // Create on enter key press
  window.addEventListener("keypress", (event) => {
    if (event.key === "Enter") {
      updateInstruction();
    }
  });
  // Delete step
  $$(".delete-step-button").forEach((el) => {
    el.addEventListener("click", () => {
      const stepId = parseInt(el.value);
      deleteStep(stepId);
    });
  });

  // Toggle update step container
  $$(".toggle-update-step").forEach((el) => {
    el.addEventListener("click", () => {
      const key = el.value;
      // TODO: don't store ids in the id.. use custom data directives, like `data-*`
      $(`#update-area-${key}`).classList.toggle("hidden");
    });
  });
  // update step
  $$(".update-step-button").forEach((el) => {
    el.addEventListener("click", () => {
      const stepId = parseInt(el.value);
      updateStep(stepId);
    });
  });
}
// Run `main` once the dom content is loaded
window.addEventListener("DOMContentLoaded", main);

function toggleModal() {
  let modalEl = $("#modal");
  let inputEl = $("#update-instruction-input");
  const open = !modalEl.classList.contains("opacity-0");
  if (open) {
    // open -> closed
    setTimeout(() => {
      // TODO: reset after modal close
    }, 150); // 150 ms is the same as the default transition time in tailwindcss
  } else {
    // closed -> open
    inputEl.focus();
    inputEl.select();
  }
  // Fade in the modal
  modalEl.classList.toggle("pointer-events-none");
  modalEl.classList.toggle("opacity-0");
}

async function updateInstruction() {
  try {
    const response = await fetch("/instruction", {
      method: "PUT",
      headers: {
        Accept: "application/json",
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        title: $("#update-instruction-input").value,
        id: parseInt(window.location.pathname.split("/").pop()),
      }),
    });
    const json = await response.json();

    // If successful, reload page
    if (response.status === 200) {
      window.location.href = `/instruction/${json.id}`;
    }

    // Check for validation error
    if (response.status === 422) {
      // Reset the input to original value
      $("#update-instruction-input").value = json.input;
      // Display validation error
      $("#update-instruction-error").textContent = json.msg;
    }
  } catch (error) {
    $("#update-instruction-error").textContent = error;
  }
}

async function createStep() {
  let title = $("#create-step-title").value;
  let minutes = $("#create-step-minutes").value;
  let secondsField = $("#create-step-seconds").value;
  let instructionId = parseInt(window.location.pathname.split("/").pop());
  let seconds = parseInt(minutes) * 60 + parseInt(secondsField);
  try {
    const response = await fetch("/step", {
      method: "POST",
      headers: {
        Accept: "application/json",
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        instructionId,
        title,
        seconds,
      }),
    });
    const json = await response.json();

    // If successful, reload page
    if (response.status === 200) {
      window.location.href = `/instruction/${instructionId}`;
    }

    // Check for validation error
    if (response.status === 422) {
      // Reset the input to original value
      $("#create-step-title").value = json.input;
      // Display validation error
      $("#create-step-error").textContent = json.msg;
    }
  } catch (error) {
    $("#create-step-error").textContent = error;
  }
}

// id might have to be an argument from the button click value
async function deleteStep(stepId) {
  try {
    const response = await fetch("/step", {
      method: "DELETE",
      headers: {
        Accept: "application/json",
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        // TODO: rename to step_id on server
        id: stepId,
      }),
    });
    const _json = await response.json();
    // If successful, reload page
    if (response.status === 200) {
      const instructionId = parseInt(window.location.pathname.split("/").pop());
      window.location.href = `/instruction/${instructionId}`;
    }
    // Check for validation error
    if (response.status === 422) {
      // TODO
    }
  } catch (error) {
    // TODO
  }
}

async function updateStep(stepId) {
  let title = $("#update-step-title").value;
  let minutes = $("#update-step-minutes").value;
  let seconds = $("#update-step-seconds").value;

  console.log({ title, minutes, id: stepId });
  try {
    const response = await fetch("/step", {
      method: "PUT",
      headers: {
        Accept: "application/json",
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        id: stepId,
        title,
        seconds: parseInt(minutes) * 60 + parseInt(seconds),
      }),
    });
    const json = await response.json();

    // If successful, reload page
    if (response.status === 200) {
      let instructionId = parseInt(window.location.pathname.split("/").pop());
      window.location.href = `/instruction/${instructionId}`;
    }

    // Check for validation error
    if (response.status === 422) {
      // Reset the input to original value
      $("#create-step-title").value = json.input;
      // Display validation error
      $("#create-step-error").textContent = json.msg;
    }
  } catch (error) {
    $("#create-step-error").textContent = error;
  }
}
