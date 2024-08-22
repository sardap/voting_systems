<script setup lang="ts">
import type {
  GenericElection,
  GenericElectionResult,
  QuotaPreferentialVicLabor2024Result
} from '@/backend'
import { ref, type PropType } from 'vue'
import DecisionLog from './DecisionLog.vue'

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

const result = ref(props.result as QuotaPreferentialVicLabor2024Result)
</script>

<template>
  <div>
    <div>
      <h3>Summary</h3>
      <p>Quota: {{ result.result.quota }}</p>
      <p>
        Elected: {{ result.result.elected_candidates.map((i) => result.candidates[i]).join(', ') }}
      </p>
    </div>
    <div>
      <h3>Decisions</h3>
      <DecisionLog :decision_log="result.result.decision_log" />
    </div>
  </div>
</template>

<style scoped></style>
