<script setup lang="ts">
import type { GenericElection, SinglePartyResult, GenericElectionResult } from '@/backend'
import { ref, type PropType } from 'vue'

const props = defineProps({
  election: {
    type: Object as PropType<GenericElection>,
    required: true
  },
  result: {
    type: Object as PropType<GenericElectionResult>,
    required: true
  }
})

const result = ref(props.result as SinglePartyResult)
</script>

<template>
  <h2>Total Votes: {{ result.vote_count }}</h2>
  <h2>
    Filled Votes: {{ ((result.filled_votes / result.vote_count) * 100).toFixed(2) }}% ({{
      result.filled_votes
    }})
  </h2>
  <h2>
    Blank Votes: {{ ((result.blank_votes / result.vote_count) * 100).toFixed(2) }}% ({{
      result.blank_votes
    }})
  </h2>
  <h2>Elected: {{ result.won ? '🏆 Yes 🏆' : '😞 Yes but less than 50% 😞' }}</h2>
</template>

<style scoped></style>
