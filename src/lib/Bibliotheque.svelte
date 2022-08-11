<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  invoke("fetch_all").then((vec) => console.log(vec));

  let promise = getAllBooks();

  async function getAllBooks(): Promise<Array<Object>> {
    const vec: Array<Object> = await invoke("fetch_all");
    return vec;
  }

  function reloadOnClick() {
    promise = getAllBooks();
  }
</script>

<main>
  <div class="min-h-full">
    <header class="bg-white shadow">
      <div class="max-w-7xl mx-auto py-6 px-4 sm:px-6 lg:px-8">
        <h1 class="text-3xl font-bold text-gray-900">Bibliothèque</h1>
      </div>
      <button on:click={reloadOnClick}> reload </button>
    </header>
    <main>
      {#await promise}
        <p>Waiting...</p>
      {:then all_books}
        {#each all_books as { title, year }, i}
          <li>{title}, {year}</li>
        {/each}
      {/await}
      <div class="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8" />
    </main>
  </div>
</main>
