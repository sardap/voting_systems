<script setup lang="ts">
import { ref } from 'vue';
import ElectionTypePicker from './ElectionTypePicker.vue';
import { ElectionType, election_type_to_path, create_generic_election } from '@/backend';
import type { StvCreateElection } from '@/backend';
import PrefElectionCreate from './election/preferential_voting/PrefElectionCreate.vue';
import SvtCreate from './election/single_transferable_vote/SvtCreate.vue';
import CreateElectionBase from './CreateElectionBase.vue';
import BordaCountCreate from './election/borda_count/BordaCountCreate.vue';
import ApprovalCreate from './election/approval/ApprovalCreate.vue';
import StarCreate from './election/star/StarCreate.vue';
import CumulativeCreate from './election/cumulative/CumulativeCreate.vue';
import AntiPluralityCreate from './election/anti_plurality/AntiPluralityCreate.vue';
import SinglePartyCreate from './election/single_party/SinglePartyCreate.vue';
import ThreeTwoOneCreate from './election/three_two_one/ThreeTwoOneCreate.vue';
import CondorcetMethodCreate from './election/condorcet_method/CondorcetMethodCreate.vue';
import MajorityJudgmentCreate from './election/majority_judgment/MajorityJudgmentCreate.vue';
import ScoreCreate from './election/score/ScoreCreate.vue';
import UsualJudgmentCreate from './election/usual_judgment/UsualJudgmentCreate.vue';
import SingleNonTransferableCreate from './election/single_non_transferable/SingleNonTransferableCreate.vue';
import CreateCandidates from './CreateCandidates.vue';
import InfoCom from './InfoCom.vue';

const props = defineProps({
    api_key: {
        type: String,
        required: true,
    }
});

const selected_election_type = ref<ElectionType>(ElectionType.PreferentialVoting);
const to_create = ref({
    title: '',
    require_token: false,
    options: ['', ''],
    elected_count: 1,
    max_score: 10,
    max_votes: 100,
});
const loading = ref(false);
const error = ref<String | null>(null);
const refresh_create = ref(0);


function refresh_to_create() {
    error.value = null;
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
            break;
        case ElectionType.SingleParty:
            to_create.value.options = [to_create.value.options[0]];
            console.log(`to_create ${JSON.stringify(to_create.value)}`);
            break;
        case ElectionType.ThreeTwoOne:
            while (to_create.value.options.length < 3) {
                to_create.value.options.push('');
            }
            break;
    }
    refresh_create.value++;
}

interface CreateElectionResponse {
    id: string;
    key: string;
}

async function submit() {
    if (selected_election_type.value == ElectionType.FirstPastThePost) {
        error.value = "Are you stupid? I give you a bunch of good voting systems and you choose the worst one?";
        return;
    }

    console.log(`candidates ${JSON.stringify(to_create.value.options)}`);

    if (!props.api_key) {
        error.value = "API key is required";
        return;
    }

    if (to_create.value.title.length == 0) {
        error.value = "Title is required";
        return;
    }

    if (selected_election_type.value != ElectionType.SingleParty) {
        if (to_create.value.options.length <= 1) {
            error.value = "At least two candidates is required";
            return;
        }

    }

    if (to_create.value.options.some(candidate => candidate.length == 0)) {
        error.value = "Not a single candidate name cannot be empty";
        return;
    }

    switch (selected_election_type.value) {
        case ElectionType.PreferentialVoting:
            break;
        case ElectionType.SingleTransferableVote:
            const stv_to_create = to_create.value as StvCreateElection;

            if (stv_to_create.elected_count < 1) {
                error.value = "At least one candidate must be elected";
                return;
            }
            break;
        case ElectionType.BordaCount:
            break;
        case ElectionType.Approval:
            break;
        case ElectionType.Star:
            break;
        case ElectionType.Cumulative:
            if (to_create.value.max_votes < 1) {
                error.value = "At least one vote must be allowed";
                return;
            }
            break;
        case ElectionType.AntiPlurality:
            break;
        case ElectionType.SingleParty:
            break;
        case ElectionType.ThreeTwoOne:
            if (to_create.value.options.length < 3) {
                error.value = "At least three candidates are required";
                return;
            }
            break;
        case ElectionType.CondorcetMethod:
            break;
        case ElectionType.Score:
            if (to_create.value.max_score < 1) {
                error.value = "At least one point must be allowed";
                return;
            }

            if (to_create.value.max_score > 100) {
                error.value = "Max score is too high";
                return;
            }

            break;
    }

    if (error.value) {
        return;
    }

    loading.value = true;

    const response = await create_generic_election(selected_election_type.value, props.api_key, to_create.value);

    loading.value = false;

    const data = await response.text();
    loading.value = false;
    if (!response.ok) {
        error.value = data;
        return;
    }
    const result: CreateElectionResponse = JSON.parse(data);

    const path = election_type_to_path(selected_election_type.value);
    location.href = `/${path}/${result.id}/results?api_key=${result.key}`;
}


</script>

<template>
    <div>
        <p v-if="loading">Loading</p>
        <div v-else>
            <InfoCom :key="selected_election_type" :election_type="selected_election_type" />
            <br />
            <p>Election Type</p>
            <ElectionTypePicker v-model="selected_election_type" @change="refresh_to_create" />
            <br />
            <div v-if="error">
                <p class="error-text">Error: {{ error }}</p>
                <br />
            </div>
            <CreateElectionBase v-model="to_create" />
            <br />
            <div :key="refresh_create">
                <div v-if="selected_election_type == ElectionType.FirstPastThePost">
                    <CreateCandidates v-model="to_create.options" />
                </div>
                <PrefElectionCreate v-else-if="selected_election_type == ElectionType.PreferentialVoting"
                    v-model="to_create" />
                <SvtCreate v-else-if="selected_election_type == ElectionType.SingleTransferableVote"
                    v-model="to_create as StvCreateElection" />
                <BordaCountCreate v-else-if="selected_election_type == ElectionType.BordaCount" v-model="to_create" />
                <ApprovalCreate v-else-if="selected_election_type == ElectionType.Approval" v-model="to_create" />
                <StarCreate v-else-if="selected_election_type == ElectionType.Star" v-model="to_create" />
                <CumulativeCreate v-else-if="selected_election_type == ElectionType.Cumulative" v-model="to_create" />
                <AntiPluralityCreate v-else-if="selected_election_type == ElectionType.AntiPlurality" v-model="to_create" />
                <SinglePartyCreate v-else-if="selected_election_type == ElectionType.SingleParty" v-model="to_create" />
                <ThreeTwoOneCreate v-else-if="selected_election_type == ElectionType.ThreeTwoOne" v-model="to_create" />
                <CondorcetMethodCreate v-else-if="selected_election_type == ElectionType.CondorcetMethod"
                    v-model="to_create" />
                <MajorityJudgmentCreate v-else-if="selected_election_type == ElectionType.MajorityJudgment"
                    v-model="to_create" />
                <ScoreCreate v-else-if="selected_election_type == ElectionType.Score" v-model="to_create" />
                <UsualJudgmentCreate v-else-if="selected_election_type == ElectionType.UsualJudgment" v-model="to_create" />
                <SingleNonTransferableCreate v-else-if="selected_election_type == ElectionType.SingleNonTransferable"
                    v-model="to_create" />
                <p v-else>Unimplemented</p>
            </div>
            <button @click="submit" class="create-button">Create</button>
        </div>
    </div>
</template>

<style scoped>
.create-button {
    padding: 20px 40px;
    margin-top: 30px;
}
</style>
