<script setup lang="ts">
import type { PrefElectionResult, GenericElectionResult, GenericElection } from '@/backend'
import { ref, type PropType } from 'vue'
import EliminationTable from './EliminationTable.vue'
import VotesTable from '@/components/results/VotesTable.vue'

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

const result = ref(props.result as PrefElectionResult)
</script>

<template>
  <div>
    <h2>Elimination Table</h2>
    <EliminationTable :election_result="result" />
    <br />
    <!-- <h2>Preference Tally</h2>
    <PreferenceTally :election_result="result" />
    <br />
    <h2>Preference Flow</h2>
    <PreferenceFlow :election_result="result" />
    <br /> -->
    <h2>Vote Table</h2>
    <VotesTable :options="result.candidates" :votes="result.votes" />
  </div>
</template>

<style scoped></style>
