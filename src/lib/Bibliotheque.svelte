<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { identity } from "svelte/internal";
  import Accueil from "./Accueil.svelte";

  invoke("fetch_all").then((vec) => console.log(vec));

  let displayID = -1;
  let displayAuthorPage = 0;
  let promise = getAllBooks();

  async function getAllBooks(): Promise<Array<any>> {
    const vec: Array<Object> = await invoke("fetch_all");
    return vec;
  }

  async function getBookById(id): Promise<any> {
    const vec: object = await invoke("fetch_one_book", { id: id });
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
    promise = getBookById(id);
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
          class="py-2 px-4 my-2  bg-indigo-600 hover:bg-indigo-700 focus:ring-indigo-500 focus:ring-offset-indigo-200 text-white max-w-4xl transition ease-in duration-200 text-center text-base font-semibold focus:outline-none focus:ring-2 focus:ring-offset-2  rounded-lg"
        >
          Actualiser
        </button>
      </div>
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
                Siècle
              </th>
            </tr>
          </thead>
          <tbody>
            {#await promise}
              <p>Waiting...</p>
            {:then all_books}
              {#each all_books as { id, title, author, century }}
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
                  <td class="border-b-2 p-4 dark:border-dark-5"> {century} </td>
                </tr>
              {/each}
            {/await}
          </tbody>
        </table>
        <div class="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8" />
      </main>
    {:else if displayAuthorPage == 0}
      {#await promise}
        Chargement...
      {:then book}
        <button
          type="button"
          on:click={returnToHome}
          class="py-2 px-4 m-2  bg-indigo-600 hover:bg-indigo-700 focus:ring-indigo-500 focus:ring-offset-indigo-200 text-white max-w-4xl transition ease-in duration-200 text-center text-base font-semibold focus:outline-none focus:ring-2 focus:ring-offset-2  rounded-lg"
        >
          &lt;&lt; Retour
        </button>
        {#each book as { id, title, author, year, century, genre, theme, place, difficulty, read, copies, meta_book, fluff }}
          <div class="shadow-lg py-12 m-4 rounded-2xl bg-sky-100">
            <div class="flex flex-col items-center justify-center p-4 -mt-16">
              <p class="text-slate-900 text-3xl font-medium mt-2">
                {title}, {author}
              </p>
              <p
                class="text-xl font-bold p-2 bg-sky-300 text-slate-600 px-4 rounded-full my-2"
              >
                {year}
              </p>
              <div class="rounded-lg p-2 w-full mt-4">
                <div class="items-center justify-between text-sm text-gray-600">
                  <p class="flex flex-col text-xl mb-4">
                    Siècle
                    <span class="text-slate-900 text-xl italic">
                      {century}
                    </span>
                  </p>
                  <p class="flex flex-col text-xl mb-4">
                    Genre
                    <span class="text-slate-900 text-xl italic">
                      {genre}
                    </span>
                  </p>
                  <p class="flex flex-col text-xl mb-4">
                    Thème
                    <span class="text-slate-900 text-xl italic">
                      {theme}
                    </span>
                  </p>
                  <p class="flex flex-col text-xl mb-4">
                    Lieu
                    <span class="text-slate-900 text-xl italic">
                      {place}
                    </span>
                  </p>
                  <p class="flex flex-col text-xl mb-4">
                    Difficulté
                    <span class="text-slate-900 text-xl italic">
                      {difficulty}/5
                    </span>
                  </p>
                  <p class="flex flex-col text-xl mb-4">
                    Lu
                    <span class="text-slate-900 text-xl italic">
                      {read}
                    </span>
                  </p>
                  <p class="flex flex-col text-xl mb-4">
                    Nombre d'exemplaires
                    <span class="text-slate-900 text-xl italic">
                      {copies}
                    </span>
                  </p>
                  <p class="flex flex-col text-xl mb-4">
                    Livre du Prof
                    <span class="text-slate-900 text-xl italic">
                      {meta_book}
                    </span>
                  </p>

                  <p class="flex flex-col text-xl mb-4">
                    Commentaires
                    <span class="text-slate-900 text-xl italic">
                      {fluff}
                    </span>
                  </p>
                </div>
              </div>
            </div>
          </div>
        {/each}
      {/await}
    {:else}
      auteur
    {/if}
  </div>
</main>
