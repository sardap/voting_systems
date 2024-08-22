import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'
import { ElectionType, election_type_to_path } from '@/backend'

import VoteView from '@/views/VoteView.vue'
import ResultViewVue from '@/views/ResultView.vue'
import TestView from '@/views/TestView.vue'

function create_vote_page(type: ElectionType) {
  const path = election_type_to_path(type)
  return {
    path: `/${path}/:id`,
    name: `${type} Election`,
    component: VoteView,
    props: { election_type: type }
  }
}

function create_result_page(type: ElectionType) {
  const path = election_type_to_path(type)
  return {
    path: `/${path}/:id/results`,
    name: `${type} Election Result`,
    component: ResultViewVue,
    props: { election_type: type }
  }
}

const vote_pages = Object.values(ElectionType).map((type) => create_vote_page(type))
const result_pages = Object.values(ElectionType).map((type) => create_result_page(type))

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'Home',
      component: HomeView
    },
    {
      path: '/create',
      name: 'Create',
      component: () => import('../views/CreateView.vue')
    },
    {
      path: '/test',
      name: 'Test',
      component: TestView
    },
    ...vote_pages,
    ...result_pages
  ]
})

export default router
