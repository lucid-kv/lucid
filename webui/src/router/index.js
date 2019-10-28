import Vue from 'vue'
import VueRouter from 'vue-router'

import store from '@/store'
import Home from '@/views/Home.vue'

Vue.use(VueRouter)

const routes = [
  {
    path: '/',
    name: 'Home',
    component: Home
  },
  {
    path: '/login',
    name: 'Login',
    component: () => import('@/views/Login.vue')
  },
  {
    path: '/kv',
    name: 'KvProofOfConcept',
    component: () => import('@/views/KvProofOfConcept.vue')
  }
]

const router = new VueRouter({ routes })

// Restrict route navigation depending on logged in current state
router.beforeEach((to, from, next) => {
  if (!store.getters.isLoggedIn) {
    if (to.name !== 'Login')
      return next({ name: 'Login' })
  }
  else {
    if (to.name === 'Login')
      return next({ name: 'Home' })
  }
  next()
})

export default router
