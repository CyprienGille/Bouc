<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import Livre from "./Livre.svelte";

  // invoke("fetch_all").then((vec) => console.log(vec));

  let displayID = -1;
  let displayAuthorPage = 0;
  let promise = getAllBooks();
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

  async function getAllBooks(): Promise<Array<any>> {
    const vec: Array<Object> = await invoke("fetch_all");
    return vec;
  }

  function reloadOnClick() {
    promise = getAllBooks();
  }

  function returnToHome() {
    displayID = -1;
    reloadOnClick();
  }

  function OnClickOnBook(id) {
    displayID = id;
  }

  function submitSearch() {
    promise = search();
  }

  async function search(): Promise<Array<any>> {
    const fieldTxt = (<HTMLInputElement>document.getElementById("field-choice"))
      .value;
    const field = txtToField.get(fieldTxt);
    const substring = (<HTMLInputElement>(
      document.getElementById("input-search")
    )).value;

    const vec: Array<Object> = await invoke("fetch_if_contains", {
      fieldName: field,
      substring: substring,
    });
    return vec;
  }
  // function OnClickOnAuthor(name) {
  //   displayAuthorPage = 1;
  //   promise = getBooksFromAuthor(name);
  // }
</script>

<main>
  <div class="min-h-full">
    {#if displayID == -1}
      <header class="bg-sky-50 shadow">
        <div class="max-w-7xl mx-auto py-6 px-4 sm:px-6 lg:px-8">
          <h1 class="text-3xl font-bold text-gray-900">Bibliothèque</h1>
        </div>
      </header>
      <div class="text-center">
        <button
          type="button"
          on:click={reloadOnClick}
          class="py-2 px-4 my-2  bg-sky-600 hover:bg-indigo-700 focus:ring-sky-500 focus:ring-offset-indigo-200 text-white max-w-4xl transition ease-in duration-200 text-center text-base font-semibold focus:outline-none focus:ring-2 focus:ring-offset-2  rounded-lg"
        >
          Actualiser
        </button>
      </div>
      <form class="w-full max-w-4xl">
        <div class="flex flex-wrap -mx-3 mb-3">
          <div class="w-full md:w-1/3 px-3 mb-6 md:mb-0">
            <select
              id="field-choice"
              class="block w-full bg-gray-200 border border-gray-200 text-gray-700 py-3 px-4 pr-5 rounded leading-tight focus:outline-none focus:bg-white focus:border-gray-500"
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
          <div class="w-full md:w-1/3 px-3 mb-6 md:mb-0">
            <input
              type="text"
              id="input-search"
              class="appearance-none block w-full h-full bg-gray-200 text-gray-700 border border-gray-200 rounded px-4 leading-tight focus:outline-none focus:bg-white focus:border-gray-500"
            />
          </div>
          <div class="w-full md:w-1/3 px-3 mb-6 md:mb-0 text-center">
            <button
              type="button"
              on:click={submitSearch}
              class="h-full px-10 bg-sky-600 hover:bg-indigo-700 focus:ring-sky-500 focus:ring-offset-indigo-200 text-white transition ease-in duration-200 text-center text-base font-semibold focus:outline-none focus:ring-2 focus:ring-offset-2  rounded-lg"
            >
              Rechercher
            </button>
          </div>
        </div>
      </form>
      <main>
        <table class="table bg-white shadow rounded-lg w-full">
          <thead>
            <tr>
              <th
                class="border-b-2 p-4 dark:border-dark-5 whitespace-nowrap font-normal text-gray-900"
              >
                N°
              </th>
              <th
                class="border-b-2 p-4 dark:border-dark-5 whitespace-nowrap font-normal text-gray-900"
              >
                Titre
              </th>
              <th
                class="border-b-2 p-4 dark:border-dark-5 whitespace-nowrap font-normal text-gray-900"
              >
                Auteur
              </th>
              <th
                class="border-b-2 p-4 dark:border-dark-5 whitespace-nowrap font-normal text-gray-900"
              >
                Année
              </th>
            </tr>
          </thead>
          <tbody>
            {#await promise}
              <p>Waiting...</p>
            {:then all_books}
              {#each all_books as { id, title, author, year }}
                <tr class="text-gray-700 text-center">
                  <td class="border-b-2 p-4 dark:border-dark-5"> {id} </td>
                  <td class="border-b-2 p-4 dark:border-dark-5">
                    <button
                      type="button"
                      class="text-sky-500 underline"
                      on:click={() => OnClickOnBook(id + "")}
                    >
                      {title}
                    </button>
                  </td>
                  <td class="border-b-2 p-4 dark:border-dark-5">
                    <button type="button" class="text-sky-500 underline">
                      {author}
                    </button>
                  </td>
                  <td class="border-b-2 p-4 dark:border-dark-5"> {year} </td>
                </tr>
              {/each}
            {/await}
          </tbody>
        </table>
        <div class="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8" />
      </main>
    {:else if displayAuthorPage == 0}
      <button
        type="button"
        on:click={returnToHome}
        class="py-2 px-4 m-2  bg-indigo-600 hover:bg-indigo-700 focus:ring-indigo-500 focus:ring-offset-indigo-200 text-white max-w-4xl transition ease-in duration-200 text-center text-base font-semibold focus:outline-none focus:ring-2 focus:ring-offset-2  rounded-lg"
      >
        &lt;&lt; Retour
      </button>
      <Livre {displayID} />
    {:else}
      auteur
    {/if}
  </div>
</main>
