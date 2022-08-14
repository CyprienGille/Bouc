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
      {#if displayID == -1}
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
      {:else if displayAuthorPage == 0}
        {#await promise}
          Chargement...
        {:then book}
          {#each book as { id, title, author, year }}
            <div>{id}</div>
            <div>{title}</div>
            <div>{author}</div>
            <div>{year}</div>
          {/each}
        {/await}
      {:else}
        auteur
      {/if}
      <div class="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8" />
    </main>
  </div>
</main>
