import {createRouter, createWebHistory} from 'vue-router'
import SummarizationView from "@/views/SummarizationView.vue";
import OnnxView from "@/views/OnnxView.vue";
import Classification from "@/views/Classification.vue";
import TransferClassification from "@/views/TransferClassification.vue";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      // component: HomeView
      component: SummarizationView
    },
    {
      path: '/onnx',
      name: 'onnx',
      // component: HomeView
      component: OnnxView
    },
    {
      path: '/classification',
      name: 'Classification',
      // component: HomeView
      component: Classification
    },
    {
      path: '/transfer',
      name: 'transfer',
      // component: HomeView
      component: TransferClassification
    },
    {
      path: '/about',
      name: 'about',
      // route level code-splitting
      // this generates a separate chunk (About.[hash].js) for this route
      // which is lazy-loaded when the route is visited.
      component: () => import('../views/AboutView.vue')
    }
  ]
})

export default router
