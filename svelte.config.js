import preprocess from 'svelte-preprocess'

const config = {
  // options passed to svelte.compile (https://svelte.dev/docs#compile-time-svelte-compile)
  compilerOptions: {},

  // an array of file extensions that should be treated as Svelte components
  extensions: ['.svelte'],

  // options passed to svelte.preprocess (https://svelte.dev/docs#compile-time-svelte-preprocess)
  preprocess: [
    preprocess({
      postcss: true,
    }),
  ],
};


export default config;