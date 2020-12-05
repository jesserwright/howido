window.addEventListener("load", () => {
    let $ = document.querySelector.bind(document);

    $("#create-step-button").addEventListener("click", createStep);
  
    function displayError(msg) {
      $("#create-step-input-error").textContent = msg;
    }
  
    async function createStep() {
      let title = $("#create-step-title").value;
      let minutes = $("#create-step-mintues").value;
      let secondsField = $("#create-step-seconds").value;
  
      let seconds = minutes * 60 + secondsField;
  
      try {
        const response = await fetch("/step", {
          method: "post",
          headers: {
            Accept: "application/json",
            "Content-Type": "application/json",
          },
          body: JSON.stringify({ title, seconds }),
        });
        const json = await response.json();
        console.log(json);
  
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
          // Clear the input field
          $("#create-step-title").value = "";
        }
      } catch (error) {
        // Display a low-level error (like network - a fetch failure)
        displayError(error);
      }
    }
  });