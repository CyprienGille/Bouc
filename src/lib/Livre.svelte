<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  export let displayID;
  let modifying = 0;
  let promise = getBookById(displayID);
  let inputType = 0;
  let txtToField = new Map([
    ["Titre", "title"],
    ["Auteur", "author"],
    ["Année", "year"],
    ["Genre", "genre"],
    ["Thème", "theme"],
    ["Lieu", "place"],
    ["Difficulté", "difficulty"],
    ["Lu", "read"],
    ["Copies", "copies"],
    ["Livre du Prof", "meta_book"],
    ["Commentaires", "fluff"],
  ]);

  async function getBookById(id): Promise<any> {
    const vec: object = await invoke("fetch_one_book", { id: id });
    return vec;
  }

  function openModifyPrompt() {
    modifying = 1;
  }

  function changeInputType() {
    const chosenField = (<HTMLInputElement>(
      document.getElementById("field-choice")
    )).value;
    if (chosenField == "Lu" || chosenField == "Livre du Prof") {
      inputType = 1;
    } else if (chosenField == "Difficulté") {
      inputType = 2;
    } else {
      inputType = 0;
    }
  }

  async function submitModification() {
    let fieldTxt = (<HTMLInputElement>document.getElementById("field-choice"))
      .value;
    let field = txtToField.get(fieldTxt);
    const success = await invoke("modify", {
      id: Number(displayID),
      fieldName: field,
      newValue: (<HTMLInputElement>document.getElementById("input-new-value"))
        .value,
    });
    modifying = 0;
    promise = getBookById(displayID);
    return success;
  }
</script>

<main>
  <div class="min-h-full">
    {#await promise}
      Chargement...
    {:then book}
      {#each book as { id, title, author, year, genre, theme, place, difficulty, read, copies, meta_book, fluff }}
        <main>
          <div class="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
            <form class="w-full max-w-4xl">
              <div class="flex flex-wrap -mx-3 mb-3">
                <div class="w-full md:w-1/2 px-3 mb-6 md:mb-0">
                  <label
                    class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2"
                    for="field-title"
                  >
                    Titre
                  </label>
                  <p
                    id="field-title"
                    class="border border-slate-500 rounded-md py-2 px-1"
                  >
                    {title}
                  </p>
                </div>
                <div class="w-full md:w-1/2 px-3">
                  <label
                    class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2"
                    for="field-author"
                  >
                    Auteur
                  </label>
                  <p
                    id="field-author"
                    class="border border-slate-500 rounded-md py-2 px-1"
                  >
                    {author}
                  </p>
                </div>
              </div>
              <div class="flex flex-wrap -mx-3 mb-3">
                <div class="w-full md:w-1/3 px-3 mb-6 md:mb-0">
                  <label
                    class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2"
                    for="field-genre"
                  >
                    Genre
                  </label>
                  <p
                    id="field-genre"
                    class="border border-slate-500 rounded-md py-2 px-1"
                  >
                    {genre}
                  </p>
                </div>
                <div class="w-full md:w-1/3 px-3">
                  <label
                    class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2"
                    for="field-theme"
                  >
                    Thème
                  </label>
                  <p
                    id="field-theme"
                    class="border border-slate-500 rounded-md py-2 px-1"
                  >
                    {theme}
                  </p>
                </div>
                <div class="w-full md:w-1/3 px-3">
                  <label
                    class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2"
                    for="field-year"
                  >
                    Année
                  </label>
                  <p
                    id="field-year"
                    class="border border-slate-500 rounded-md py-2 px-1"
                  >
                    {year}
                  </p>
                </div>
              </div>

              <div class="flex flex-wrap -mx-3 mb-2">
                <div class="w-full md:w-3/12 px-3 mb-6 md:mb-0">
                  <label
                    class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2"
                    for="field-place"
                  >
                    Lieu
                  </label>
                  <p
                    id="field-place"
                    class="border border-slate-500 rounded-md py-2 px-1"
                  >
                    {place}
                  </p>
                </div>
                <div class="w-full md:w-3/12 px-3 mb-6 md:mb-0">
                  <label
                    class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2"
                    for="field-difficulty"
                  >
                    Difficulté
                  </label>
                  <div class="relative">
                    <p
                      id="field-difficulty"
                      class="border border-slate-500 rounded-md py-2 px-1"
                    >
                      {difficulty}
                    </p>
                  </div>
                </div>
                <div class="w-full md:w-3/12 px-3 mb-6 md:mb-0">
                  <label
                    class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2"
                    for="field-copies"
                  >
                    Copies
                  </label>
                  <p
                    id="field-copies"
                    class="border border-slate-500 rounded-md py-2 px-1"
                  >
                    {copies}
                  </p>
                </div>
                <div class="w-full md:w-1/12 px-3 mb-6 md:mb-0">
                  <label
                    class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2"
                    for="field-read"
                  >
                    Lu
                  </label>
                  <p
                    class="border border-slate-500 rounded-md py-2 px-1"
                    id="field-read"
                  >
                    {read}
                  </p>
                </div>
                <div class="w-full md:w-2/12 px-3 mb-6 md:mb-0">
                  <label
                    class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2"
                    for="field-meta"
                  >
                    Livre du Prof
                  </label>
                  <p
                    class="border border-slate-500 rounded-md py-2 px-1"
                    id="field-meta"
                  >
                    {meta_book}
                  </p>
                </div>
              </div>
              <div class="flex flex-wrap -mx-3 mb-6">
                <div class="w-full px-3">
                  <label
                    class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2"
                    for="field-fluff"
                  >
                    Commentaires
                  </label>
                  <p
                    id="field-fluff"
                    class="border border-slate-500 rounded-md py-5 px-1"
                  >
                    {fluff}
                  </p>
                </div>
              </div>
            </form>
          </div>
        </main>
      {/each}
    {/await}
    {#if modifying == 0}
      <button
        type="button"
        class="py-2 px-4  bg-slate-500 hover:bg-slate-700 focus:ring-indigo-500 focus:ring-offset-indigo-200 text-white w-full transition ease-in duration-200 text-center text-base font-semibold shadow-md focus:outline-none focus:ring-2 focus:ring-offset-2  rounded-lg "
        on:click={openModifyPrompt}
      >
        Modifier
      </button>
    {:else}
      <button
        type="button"
        class="py-2 px-4  bg-slate-500 hover:bg-slate-700 focus:ring-indigo-500 focus:ring-offset-indigo-200 text-white w-full transition ease-in duration-200 text-center text-base font-semibold shadow-md focus:outline-none focus:ring-2 focus:ring-offset-2  rounded-lg "
        on:click={submitModification}
      >
        Valider
      </button>
      <form class="w-full">
        <div class="flex flex-wrap">
          <div class="w-full md:w-1/2 px-3">
            <label
              class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2"
              for="field-choice"
            >
              Champ à modifier
            </label>
            <select
              id="field-choice"
              class="block w-full bg-gray-200 border border-gray-200 text-gray-700 py-3 px-4 pr-5 rounded leading-tight focus:outline-none focus:bg-white focus:border-gray-500"
              on:change={changeInputType}
            >
              <option>Titre</option>
              <option>Auteur</option>
              <option>Année</option>
              <option>Genre</option>
              <option>Thème</option>
              <option>Lieu</option>
              <option>Difficulté</option>
              <option>Lu</option>
              <option>Copies</option>
              <option>Livre du Prof</option>
              <option>Commentaires</option>
            </select>
          </div>

          <div class="w-full md:w-1/2 px-3">
            <label
              class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2"
              for="input-new-value"
            >
              Nouvelle valeur
            </label>
            {#if inputType == 1}
              <select
                id="input-new-value"
                class="block w-full bg-gray-200 border border-gray-200 text-gray-700 py-3 px-4 pr-5 rounded leading-tight focus:outline-none focus:bg-white focus:border-gray-500"
              >
                <option>true</option>
                <option>false</option>
              </select>
            {:else if inputType == 2}
              <select
                id="input-new-value"
                class="block w-full bg-gray-200 border border-gray-200 text-gray-700 py-3 px-4 pr-5 rounded leading-tight focus:outline-none focus:bg-white focus:border-gray-500"
              >
                <option>1</option>
                <option>2</option>
                <option>3</option>
                <option>4</option>
                <option>5</option>
              </select>
            {:else}
              <input
                type="text"
                id="input-new-value"
                class="appearance-none block w-full bg-gray-200 text-gray-700 border border-gray-200 rounded py-3 px-4 leading-tight focus:outline-none focus:bg-white focus:border-gray-500"
              />
            {/if}
          </div>
        </div>
      </form>
    {/if}
  </div>
</main>
