// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  build: {

  },

  router: {
    options: {
      hashMode: true,
    }
  },

  app: {
    baseURL: '',
    cdnURL: './'
  },

  ssr: false,

  typescript: {
    strict: true,
    typeCheck: true
  },

  devtools: { enabled: true },
  modules: ["vuetify-nuxt-module", '@nuxt/icon'],

  icon: {
    customCollections: [
      {
        prefix: 'cust-icon',
        dir: './assets/icons'
      },
    ],
  },

  runtimeConfig: {
    public: {
      apiBase: 'http://localhost:8888'
    },
  },

  css: ['video.js/dist/video-js.css'],

  vuetify: {
    /* vuetify options */
    vuetifyOptions: {
      theme: {
        // defaultTheme: 'dark'
      },
    },
    moduleOptions: {
      /*
      
      treeshaking: true,
      useIconCDN: false,
 
      
      styles: true,
      autoImport: true,
      useVuetifyLabs: true,
      */
    },

  },

  features: {
    inlineStyles: false
  },

  compatibilityDate: '2024-12-13',
})