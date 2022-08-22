import { invoke } from "@tauri-apps/api";
import { Event as TauriEvent, listen, emit } from "@tauri-apps/api/event";
const $ = document.querySelector.bind(document);

document.addEventListener("DOMContentLoaded", async function () {
  const loaded = (await invoke("dom_loaded")) as string;

  // get the elements
  const pingEl = $("backend-ping") as HTMLElement;

  let pingDebounce = 0;
  // listen backend-ping event
  listen("backend-ping", function (evt: TauriEvent<any>) {
    pingDebounce += 1;
    if (!pingEl.classList.contains("on")) {
      pingEl.classList.add("on");
    }
    setTimeout(function () {
      pingDebounce -= 1;
      if (pingDebounce == 0) {
        pingEl.classList.remove("on");
      }
    }, 80);
  });

  const start_button = $("#start-button") as HTMLElement;
  const start_modal = $("#start-modal") as HTMLElement;

  const game_over_modal = $("#game-over-modal") as HTMLElement;

  const game_grid = $("#game-grid") as HTMLElement;

  game_over_modal.style.setProperty("display", "none");

  start_button.addEventListener("click", async function startGameListener() {
    console.log("Start game clicked");
    start_modal.style.setProperty("display", "none");
    const game: any = await invoke("start_game");
    console.log(game);
    game_grid.style.setProperty(
      "grid-template",
      `repeat(${game.height}, auto) / repeat(${game.width}, auto)`
    );
    draw_game(game);
  });

  listen("tick", async function tick_update(evt: TauriEvent<any>) {
    console.log("tick");
    draw_game(evt.payload)

  });

  function draw_game(game: any) {
    game_grid.innerHTML = "";
    for (let y = 0; y < game.height; y++) {
      for (let x = 0; x < game.width; x++) {
        const div = document.createElement("div");
        div.classList.add("snake-game-cell");
        game_grid.appendChild(div);

        if (game.snake.body.some((pos: any) => pos.x === x && pos.y === y)) {
          div.innerText = "🟩";
          if (game.snake.body[0].x === x && game.snake.body[0].y === y) {
            div.innerText = "🟥";
          }
          // div.innerText = "a";
        } else if (game.food.x === x && game.food.y === y) {
          div.innerText = "🍎";
          // div.innerText = "A";
        } else {
          div.innerText = " ";
          // div.innerText = ".";
        }
      }
    }
  }

  // listen("start_game", function (event: TauriEvent<any>) {
  //   console.log(event.payload);

  //   game_grid.style.setProperty(
  //     "grid-template",
  //     `repeat(${event.payload.height}, auto) / repeat(${event.payload.width}, auto)`
  //   );
  // });

  // // counter button click
  // counterButtonEl.addEventListener("pointerup", async function () {
  //   const result = (await invoke("add_count", { num: 3 })) as string;
  //   counterResultEl.textContent = result;
  // });

  // // hello click
  // helloEl.addEventListener("pointerup", async function () {
  //   const result = (await invoke("hello_world")) as string;
  //   helloEl.textContent = result;
  //   setTimeout(function () {
  //     helloEl.textContent = "Click again";
  //   }, 1000);
  // });
});
