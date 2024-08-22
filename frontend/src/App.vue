<script setup lang="ts">
import { onMounted } from 'vue'
import { RouterView } from 'vue-router'
import { v4 as uuid } from 'uuid'
import { useCookies } from 'vue3-cookies'

const { cookies } = useCookies()

onMounted(() => {
  document.title = 'THE PLACE TO VOTE'

  if (!cookies.isKey('pref_election_uuid')) {
    cookies.set('pref_election_uuid', uuid(), undefined, '/', undefined, undefined, 'Lax')
  }
  let d = new Date()
  d.setDate(d.getDate() + 365)
  cookies.set('pref_election_uuid', uuid(), d, '/*', undefined, undefined, 'Lax')
})
</script>

<template>
  <header>
    <div>
      <h1 class="title">THE PLACE TO VOTE</h1>
    </div>
  </header>

  <RouterView />
</template>

<style scoped>
div {
  max-width: 1200px;
  margin: 0 auto;
}
</style>
