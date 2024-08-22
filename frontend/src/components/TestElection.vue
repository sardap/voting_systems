<script setup lang="ts">
import { ref } from 'vue'
import ElectionTypePicker from './ElectionTypePicker.vue'
import {
  ElectionType,
  type GenericElectionResult,
  test_election,
  type TestElectionBase
} from '@/backend'
import InfoCom from './InfoCom.vue'
import QuotaPreferentialVicLabor2024Test from './election/quota_preferential_vic_labor_2024/QuotaPreferentialVicLabor2024Test.vue'
import QuotaPreferentialVicLabor2024Result from './election/quota_preferential_vic_labor_2024/results/QuotaPreferentialVicLabor2024Result.vue'

const selected_election_type = ref<ElectionType>(ElectionType.QuotaPreferentialVicLabor2024)
const loading = ref(false)
const error = ref<String | null>(null)
const refresh_create = ref(0)
const request_dirty = ref(false)
const refresh_results = ref(0)
const has_error = ref(true)

const unsupported_election_types = [
  ElectionType.PreferentialVoting,
  ElectionType.FirstPastThePost,
  ElectionType.SingleTransferableVote,
  ElectionType.BordaCount,
  ElectionType.Approval,
  ElectionType.Star,
  ElectionType.Cumulative,
  ElectionType.AntiPlurality,
  ElectionType.SingleParty,
  ElectionType.ThreeTwoOne,
  ElectionType.CondorcetMethod,
  ElectionType.MajorityJudgment,
  ElectionType.Score,
  ElectionType.UsualJudgment,
  ElectionType.SingleNonTransferable
]

function refresh_to_create() {
  error.value = null
  switch (selected_election_type.value) {
    case ElectionType.PreferentialVoting:
    case ElectionType.BordaCount:
    case ElectionType.Approval:
    case ElectionType.Star:
    case ElectionType.AntiPlurality:
    case ElectionType.CondorcetMethod:
    case ElectionType.SingleTransferableVote:
    case ElectionType.Score:
    case ElectionType.MajorityJudgment:
    case ElectionType.UsualJudgment:
    case ElectionType.Cumulative:
      break
    case ElectionType.SingleParty:
      break
    case ElectionType.ThreeTwoOne:
      break
  }
  refresh_create.value++
}

const request_payload = ref<TestElectionBase>({
  election: {},
  bundles: []
})

const response = ref<GenericElectionResult | null>(null)

async function submit() {
  loading.value = true
  request_dirty.value = false
  const result = await test_election(selected_election_type.value, request_payload.value)
  console.log(result.status)
  if (result.status !== 200) {
    const text = await result.text()
    error.value = `${result.statusText} ${text}`
  } else {
    error.value = null
    response.value = await result.json()
  }
  refresh_results.value++
  loading.value = false
}
</script>

<template>
  <div>
    <div>
      <InfoCom :key="selected_election_type" :election_type="selected_election_type" />
      <br />
      <p>Election Type</p>
      <ElectionTypePicker
        v-model="selected_election_type"
        @change="refresh_to_create"
        :disabled="unsupported_election_types"
      />
      <br />
      <div v-if="error">
        <p class="error-text">Error: {{ error }}</p>
        <br />
      </div>
      <br />
      <div :key="selected_election_type">
        <QuotaPreferentialVicLabor2024Test
          v-if="selected_election_type === ElectionType.QuotaPreferentialVicLabor2024"
          @updated="
            (updated) => {
              request_payload = {
                election: updated.election,
                bundles: updated.bundles
              }
              request_dirty = true
            }
          "
          @error="(error) => (has_error = error)"
        />
        <div v-else>
          <p>Testing is not supported for this election type yet</p>
        </div>
      </div>
    </div>
    <p v-if="loading">Loading</p>
    <button
      @click="submit"
      class="create-button"
      :disabled="loading || !request_dirty || has_error"
    >
      View Results
    </button>
    <div v-if="response" :key="refresh_results">
      <h2>Results</h2>
      <QuotaPreferentialVicLabor2024Result
        v-if="selected_election_type === ElectionType.QuotaPreferentialVicLabor2024"
        :election="request_payload.election"
        :result="response"
      />
    </div>
  </div>
</template>

<style scoped>
.create-button {
  padding: 20px 40px;
  margin-top: 30px;
}
</style>
