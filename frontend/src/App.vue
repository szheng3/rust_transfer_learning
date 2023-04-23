<script setup lang="ts">
import {RouterView} from 'vue-router'
import {ref} from "vue";
import {useRouter} from 'vue-router'

const links = ref([
  'Summarization',
  'ONNX',
  'Classification',
  'Team project - classification'
]);

const router = useRouter()

const goToDashboard = (link: string) => {
  switch (link) {
    case 'Summarization':
      router.push('/')
      break;
    case 'ONNX':
      router.push('/onnx')
      break;
    case 'Classification':
      router.push('/classification')
      break;
    case 'Team project - classification':
      router.push('/transfer')
      break;
    default:
      router.push('/')
  }
}


</script>


<template>
  <v-app>

    <v-app-bar flat>
      <v-container class="fill-height d-flex align-center">


        <v-btn
            v-for="link in links"
            @click="goToDashboard(link)"
            :key="link"
            variant="text"
        >
          {{ link }}
        </v-btn>

        <v-spacer></v-spacer>

      </v-container>
    </v-app-bar>
    <notifications class="mt-10 mr-16"/>

    <v-main class="bg-grey-lighten-4">

      <RouterView/>

    </v-main>
    <v-footer
        absolute
        class=" text-center d-flex flex-column"
        flat
    >

      <div class="text-caption">
        This project, developed using Rust, comprises four components: Shuai's individual project 2, which summarizes text; individual project 3, which focuses on ONNX; individual project 4, which explores image classification with Resnet; and our team (Shuai Zheng, Yilun Wu, Yucheng Cao) project - Classification. The platform utilizes advanced AI transfer learning algorithms to classify uploaded images into four categories: normal, violence with a gun, porn, and labeled. The model is trained in Python, while the server is built in Rust.
      </div>


      <div class="text-caption">
        {{ new Date().getFullYear() }} —<a href="https://sszzz.me">Shuai Zheng</a>
      </div>

    </v-footer>
  </v-app>
</template>

<style scoped>
/*header {*/
/*  line-height: 1.5;*/
/*  max-height: 100vh;*/
/*}*/

/*.logo {*/
/*  display: block;*/
/*  margin: 0 auto 2rem;*/
/*}*/

/*nav {*/
/*  width: 100%;*/
/*  font-size: 12px;*/
/*  text-align: center;*/
/*  margin-top: 2rem;*/
/*}*/

/*nav a.router-link-exact-active {*/
/*  color: var(--color-text);*/
/*}*/

/*nav a.router-link-exact-active:hover {*/
/*  background-color: transparent;*/
/*}*/

/*nav a {*/
/*  display: inline-block;*/
/*  padding: 0 1rem;*/
/*  border-left: 1px solid var(--color-border);*/
/*}*/

/*nav a:first-of-type {*/
/*  border: 0;*/
/*}*/

/*@media (min-width: 1024px) {*/
/*  header {*/
/*    display: flex;*/
/*    place-items: center;*/
/*    padding-right: calc(var(--section-gap) / 2);*/
/*  }*/

/*  .logo {*/
/*    margin: 0 2rem 0 0;*/
/*  }*/

/*  header .wrapper {*/
/*    display: flex;*/
/*    place-items: flex-start;*/
/*    flex-wrap: wrap;*/
/*  }*/

/*  nav {*/
/*    text-align: left;*/
/*    margin-left: -1rem;*/
/*    font-size: 1rem;*/

/*    padding: 1rem 0;*/
/*    margin-top: 1rem;*/
/*  }*/
/*}*/
</style>
