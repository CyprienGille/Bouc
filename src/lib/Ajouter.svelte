<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  async function insertBook(): Promise<any> {
    const success = await invoke("init_book_and_add", {
      title: get_input("input-title"),
      author: get_input("input-author"),
      year: get_input("input-year"),
      genre: get_input("input-genre"),
      theme: get_input("input-theme"),
      place: get_input("input-place"),
      difficulty: get_input("input-difficulty"),
      read: get_input("input-read"),
      copies: get_input("input-copies"),
      metaBook: get_input("input-meta"),
      fluff: get_input("input-fluff"),
    });
    return success;
  }

  function insertBookOnClick() {
    insertBook();
  }

  function get_input(id) {
    return document.getElementById(id).value;
  }
</script>

<main>
  <div class="min-h-full">
    <header class="bg-white shadow">
      <div class="max-w-7xl mx-auto py-6 px-4 sm:px-6 lg:px-8">
        <h1 class="text-3xl font-bold text-gray-900">Ajouter un Livre</h1>
      </div>
    </header>
    <main>
      <div class="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
        <form class="w-full max-w-2xl">
          <div class="flex flex-wrap -mx-3 mb-3">
            <div class="w-full md:w-1/2 px-3 mb-6 md:mb-0">
              <label
                class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2"
                for="input-title"
              >
                Titre
              </label>
              <input
                class="appearance-none block w-full bg-gray-200 text-gray-700 border border-gray-200 rounded py-3 px-4 mb-3 leading-tight focus:outline-none focus:bg-white"
                id="input-title"
                type="text"
                placeholder="Don Juan"
              />
            </div>
            <div class="w-full md:w-1/2 px-3">
              <label
                class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2"
                for="input-author"
              >
                Auteur
              </label>
              <input
                class="appearance-none block w-full bg-gray-200 text-gray-700 border border-gray-200 rounded py-3 px-4 leading-tight focus:outline-none focus:bg-white focus:border-gray-500"
                id="input-author"
                type="text"
                placeholder="Molière"
              />
            </div>
          </div>
          <div class="flex flex-wrap -mx-3 mb-3">
            <div class="w-full md:w-1/3 px-3 mb-6 md:mb-0">
              <label
                class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2"
                for="input-genre"
              >
                Genre
              </label>
              <input
                class="appearance-none block w-full bg-gray-200 text-gray-700 rounded py-3 px-4 mb-3 leading-tight focus:outline-none focus:bg-white"
                id="input-genre"
                type="text"
                placeholder="Théâtre"
              />
            </div>
            <div class="w-full md:w-1/3 px-3">
              <label
                class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2"
                for="input-theme"
              >
                Thème
              </label>
              <input
                class="appearance-none block w-full bg-gray-200 text-gray-700 border border-gray-200 rounded py-3 px-4 leading-tight focus:outline-none focus:bg-white focus:border-gray-500"
                id="input-theme"
                type="text"
                placeholder="Amour"
              />
            </div>
            <div class="w-full md:w-1/3 px-3">
              <label
                class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2"
                for="input-year"
              >
                Année
              </label>
              <input
                class="appearance-none block w-full bg-gray-200 text-gray-700 border border-gray-200 rounded py-3 px-4 leading-tight focus:outline-none focus:bg-white focus:border-gray-500"
                id="input-year"
                type="text"
                placeholder="1682"
              />
            </div>
          </div>

          <div class="flex flex-wrap -mx-3 mb-2">
            <div class="w-full md:w-3/12 px-3 mb-6 md:mb-0">
              <label
                class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2"
                for="input-place"
              >
                Lieu
              </label>
              <input
                class="appearance-none block w-full bg-gray-200 text-gray-700 border border-gray-200 rounded py-3 px-4 leading-tight focus:outline-none focus:bg-white focus:border-gray-500"
                id="input-place"
                type="text"
                placeholder="Etagère gauche haut"
              />
            </div>
            <div class="w-full md:w-3/12 px-3 mb-6 md:mb-0">
              <label
                class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2"
                for="input-difficulty"
              >
                Difficulté
              </label>
              <div class="relative">
                <select
                  class="block appearance-none w-full bg-gray-200 border border-gray-200 text-gray-700 py-3 px-4 pr-5 rounded leading-tight focus:outline-none focus:bg-white focus:border-gray-500"
                  id="input-difficulty"
                >
                  <option>1</option>
                  <option>2</option>
                  <option>3</option>
                  <option>4</option>
                  <option>5</option>
                </select>
                <div
                  class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-2 text-gray-700"
                >
                  <svg
                    class="fill-current h-4 w-4"
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 20 20"
                    ><path
                      d="M9.293 12.95l.707.707L15.657 8l-1.414-1.414L10 10.828 5.757 6.586 4.343 8z"
                    /></svg
                  >
                </div>
              </div>
            </div>
            <div class="w-full md:w-3/12 px-3 mb-6 md:mb-0">
              <label
                class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2"
                for="input-copies"
              >
                Copies
              </label>
              <input
                class="appearance-none block w-full bg-gray-200 text-gray-700 border border-gray-200 rounded py-3 px-4 leading-tight focus:outline-none focus:bg-white focus:border-gray-500"
                id="input-copies"
                type="number"
                placeholder="1"
              />
            </div>
            <div class="w-full md:w-1/12 px-3 mb-6 md:mb-0">
              <label
                class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2"
                for="input-read"
              >
                Lu
              </label>
              <input
                class="block bg-gray-200 text-gray-700 border border-gray-200 rounded py-3 leading-tight focus:outline-none focus:bg-white focus:border-gray-500"
                id="input-read"
                type="checkbox"
              />
            </div>
            <div class="w-full md:w-2/12 px-3 mb-6 md:mb-0">
              <label
                class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2"
                for="input-meta"
              >
                Livre du Prof
              </label>
              <input
                class="block bg-gray-200 text-gray-700 border border-gray-200 rounded py-3 leading-tight focus:outline-none focus:bg-white focus:border-gray-500"
                id="input-meta"
                type="checkbox"
              />
            </div>
          </div>
          <div class="flex flex-wrap -mx-3 mb-6">
            <div class="w-full px-3">
              <label
                class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2"
                for="input-fluff"
              >
                Commentaires
              </label>
              <input
                class="appearance-none block w-full bg-gray-200 text-gray-700 border border-gray-200 rounded py-3 px-4 mb-3 leading-tight focus:outline-none focus:bg-white focus:border-gray-500"
                id="input-fluff"
                type="text"
                placeholder=""
              />
            </div>
          </div>
        </form>
      </div>
    </main>
  </div>
</main>
