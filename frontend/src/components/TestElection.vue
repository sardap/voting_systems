<script setup lang="ts">
import { ref, watch } from 'vue'
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
import router from '@/router'
import LZUTF8 from 'lzutf8'

const previous = ref<SavedTest | null>(read_previous_from_query())
const selected_election_type = ref<ElectionType>(
  previous.value?.election_type || ElectionType.QuotaPreferentialVicLabor2024
)
const loading = ref(false)
const error = ref<String | null>(null)
const refresh_create = ref(0)
const request_dirty = ref(false)
const refresh_results = ref(0)
const has_error = ref(true)
const copied = ref(false)

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

export interface SavedTest {
  election_type: ElectionType
  request_payload: TestElectionBase
}

watch(
  request_payload,
  () => {
    request_dirty.value = true
    copied.value = false
    // Push as json to the query string
    const json = JSON.stringify({
      election_type: selected_election_type.value,
      request_payload: request_payload.value
    })
    // This lib has not been updated in 10 years but works great fuck yeah
    const compressed = LZUTF8.compress(json, { outputEncoding: 'Base64' })
    router.push({
      query: {
        previous: compressed
      }
    })
  },
  {
    deep: true
  }
)

function read_previous_from_query(): SavedTest | null {
  const previous_raw = router.currentRoute.value.query.previous
  if (previous_raw) {
    const raw_str = LZUTF8.decompress(previous_raw as string, {
      inputEncoding: 'Base64',
      outputEncoding: 'String'
    }) as string
    const previous: SavedTest = JSON.parse(raw_str)
    return previous
  }

  return null
}

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

function copy_data() {
  // Copy the url to the clipboard
  const url = window.location.href
  navigator.clipboard.writeText(url)
  copied.value = true
}

function clear_data() {
  window.location.replace(window.location.pathname)
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
      <div>
        <p>Click here to copy the URL to your clipboard which contains all the voting data.</p>
        <button @click="copy_data" :disabled="copied">{{ copied ? 'COPIED' : 'COPY URL' }}</button>
        <p>Click here to clear the URL data</p>
        <button @click="clear_data">CLEAR DATA</button>
      </div>
      <br />
      <div v-if="error">
        <p class="error-text">Error: {{ error }}</p>
        <br />
      </div>
      <br />
      <div :key="selected_election_type">
        <QuotaPreferentialVicLabor2024Test
          v-if="selected_election_type === ElectionType.QuotaPreferentialVicLabor2024"
          :bundles="previous?.request_payload.bundles"
          :election="previous?.request_payload.election"
          @updated="
            (updated) => {
              request_payload = {
                election: updated.election,
                bundles: updated.bundles
              }
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
