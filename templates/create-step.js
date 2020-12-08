// $ is already defined in 'update-instruction.js'

async function main() {
  await domLoaded();
  await createButtonClicked();

  let title = $("#create-step-title").value;
  let minutes = $("#create-step-minutes").value;
  let secondsField = $("#create-step-seconds").value;

  // Get the id from the end of the url
  let instructionId = parseInt(window.location.pathname.split("/").pop());

  let seconds = parseInt(minutes) * 60 + parseInt(secondsField);

  const input = { instructionId, title, seconds };

  try {
    const response = await fetch("/step", {
      method: "post",
      headers: {
        Accept: "application/json",
        "Content-Type": "application/json",
      },
      body: JSON.stringify(input),
    });
    const json = await response.json();

    // Check for validation error
    if (response.status === 422) {
      // in this case, the response is the same as the input object,
      // plus the error message(s)
      // in this case, there could be multiple error messages:
      // some for the input of the numbers

      // TODO: consider doing multi-field validation errors, or rendering all validation errors to the same place
      // to make it easier. don't over abstract though!

      // {id: i32, title: String, seconds: }
      $("#create-step-title").value = json.input;
      // Display validation error
      displayError(json.msg);
    }
    if (response.status === 200) {

      // If successful, reload
      if (response.status === 200) {
        window.location.href = `/instruction/${instructionId}`;
      }
    }
  } catch (error) {
    // Display a low-level error (like network - a fetch failure)
    displayError(error);
  }
}

main();

function domLoaded() {
  return new Promise((resolve, _reject) => {
    window.addEventListener("DOMContentLoaded", () => {
      resolve();
    });
  });
}

function createButtonClicked() {
  return new Promise((resolve, _reject) => {
    $("#create-step-button").addEventListener("click", () => {
      resolve();
    });
  });
}

function displayError(msg) {
  $("#create-step-input-error").textContent = msg;
}
