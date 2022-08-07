<script lang="ts">

  import logo from "./assets/svelte.png";
  import settings from "./assets/settings.png";
  import Accueil from "./lib/Accueil.svelte";
  import Bibliotheque from "./lib/Bibliotheque.svelte";
  import Ajouter from "./lib/Ajouter.svelte";
  import LivreAuHasard from "./lib/LivreAuHasard.svelte";
  import { invoke } from "@tauri-apps/api/tauri";

  let displayID = 0;

  function toggleDisplay(id: number) {
    displayID = id;
  }

  invoke("test_command");
</script>

<main>
  <nav class="bg-sky-600">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      <div class="flex items-center justify-between h-16">
        <div class="flex items-center">
          <div class="flex-shrink-0">
            <img class="h-9 w-9" src={logo} alt="Logo" />
          </div>
          <div class="hidden md:block">
            <div class="ml-10 flex items-baseline space-x-4">
              <button
                on:click={() => toggleDisplay(0)}
                class="text-gray-200 focus:bg-sky-900 focus:text-white hover:bg-sky-800 hover:text-white px-3 py-2 rounded-xl font-semibold text-xl"
              >
                Accueil
              </button>
              <button
                on:click={() => toggleDisplay(1)}
                class="text-gray-200 focus:bg-sky-900 focus:text-white hover:bg-sky-800 hover:text-white px-3 py-2 rounded-xl font-semibold text-xl"
              >
                Bibliothèque
              </button>
              <button
                on:click={() => toggleDisplay(2)}
                class="text-gray-200 focus:bg-sky-900 focus:text-white hover:bg-sky-800 hover:text-white px-3 py-2 rounded-xl font-semibold text-xl"
              >
                Ajouter
              </button>
              <button
                on:click={() => toggleDisplay(3)}
                class="text-gray-200 focus:bg-sky-900 focus:text-white hover:bg-sky-800 hover:text-white px-3 py-2 rounded-xl font-semibold text-xl"
              >
                Livre Au Hasard
              </button>
            </div>
          </div>
        </div>
        <div class="hidden md:block">
          <div class="ml-4 flex items-center md:ml-6">
            <button
              type="button"
              class="bg-inherit p-1 rounded-full text-gray-400 hover:text-white focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-gray-800 focus:ring-white"
            >
              <span class="sr-only">Settings</span>
              <img class="h-8 w-8" src={settings} alt="Settings" />
            </button>
          </div>
        </div>
      </div>
    </div>
  </nav>

  {#if displayID == 0}
    <Accueil />
  {/if}
  {#if displayID == 1}
    <Bibliotheque />
  {/if}
  {#if displayID == 2}
    <Ajouter />
  {/if}
  {#if displayID == 3}
    <LivreAuHasard />
  {/if}
</main>
