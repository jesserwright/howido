// Do some styling

// Alias the verbose DOM API
const $ = document.querySelector.bind(document);
const $$ = document.querySelectorAll.bind(document);

/*

What would a very simple way to encapsulate a 'dom component' like a modal?

- Create the dom node (s)
- Provide a slot (or a few) where 'children' can exist
- Provide some css

In the end, what does it produce? What is the type of the thing that it produces?
A dom node.

- It's ok to control things with event listeners
- BUT, can functions be passed?
- Does the event listener need to be detached when the element is 'destroyed'
- Does element 'destruction' ever happen?

What are my other options? Just duplicate code?
Or make stylesheets that duplicate? Colocation seems important?

Flutter IS NOT ready for web. Not an option. For app? Yes, that's an option...
But also more work?

The IDEA:
Flutter for create, web for view. You have to get the app to upload.

Nobody uploads VIA computer anyways AND using a website to upload images is strange.
// Yeah?

And it provides some insentive to download the app, just by using the website.

... what else? It's still the open web, with SEO and all

But no janky js, maybe *no js at all*. JUST a viewer. That'd make things SO SO easy.

No more modals even.

What part of the world is changing the fastest? Software. And I think I want to be a part of that.

So what now?

I could create the view only. Stop all the CRUD stuff, and make a very nice html-tailwind rendering of what I have in figma.

It also makes people 'users' of the software BEFORE they would need to download the app.

Expedient or meaningful?

What if this thing was just my blog?

I make the frikin CMS however I want.

Maybe I could do what I'm confident about: the layout

Completely static. What would that hurt, starting with the end?

What other problems does that seem to solve?

Compilation. If the CMS and the view are separate, then they won't need to recompile together all the time.
also, the view can be developed with static content pretty easily, as simple templates, then later 'hydrated' when it comes to that point in the dev process
Maybe also an advantage of having the 'layout' mindset for a solid session

Also dart has types, js does not.

>>>>build the thing that YOU want to use.

*/

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
