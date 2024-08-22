<script setup lang="ts">
import { useRoute } from 'vue-router'
import CreateElection from '@/components/CreateElection.vue'
import { ref } from 'vue'
import router from '@/router'

const route = useRoute()
const api_key = ref(get_api_key())

function get_api_key(): string {
  if (typeof route.query.api_key === 'string') {
    return route.query.api_key
  }

  router.push({
    query: {
      api_key: 'temp'
    }
  })

  return 'temp'
}
</script>

<template>
  <div>
    <h1>Create an election</h1>
    <CreateElection :api_key="api_key" />
  </div>
</template>

<style></style>
