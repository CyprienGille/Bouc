<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  // invoke("fetch_all").then((vec) => console.log(vec));

  export let displayID;
  let promise = getBookById(displayID);

  async function getBookById(id): Promise<any> {
    const vec: object = await invoke("fetch_one_book", { id: id });
    return vec;
  }
</script>

<main>
  <div class="min-h-full">
    {#await promise}
      Chargement...
    {:then book}
      {#each book as { id, title, author, year, century, genre, theme, place, difficulty, read, copies, meta_book, fluff }}
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
  </div>
</main>
