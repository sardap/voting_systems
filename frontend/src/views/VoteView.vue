<script setup lang="ts">
import { useRoute } from 'vue-router'
import VoteCom from '@/components/voting/VoteCom.vue'
import type { ElectionType } from '@/backend'
import type { PropType } from 'vue'

defineProps({
  election_type: {
    type: Object as PropType<ElectionType>,
    required: true
  }
})

const route = useRoute()
const election_id = route.params.id as string
const vote_token = route.query.vote_token as string
</script>

<template>
  <div>
    <VoteCom :election_id="election_id" :vote_token="vote_token" :election_type="election_type" />
  </div>
</template>

<style></style>
