<script setup lang="ts">
import { onMounted, ref, type PropType } from 'vue';
import { get_generic_election, ElectionType, type GenericElection, type CumulativeElection, type ScoreElection } from '@/backend';
import { options_to_vote_options, type VoteOption } from '@/utils';
import BordaCountVotingForm from '@/components/election/borda_count/BordaCountVotingForm.vue';
import PrefElectionVotingForm from '@/components/election/preferential_voting/PrefElectionVotingForm.vue';
import SvtVotingForm from '@/components/election/single_transferable_vote/SvtVotingForm.vue';
import ApprovalVotingForm from '@/components/election/approval/ApprovalVotingForm.vue';
import StarVotingForm from '@/components/election/star/StarVotingForm.vue';
import CumulativeVotingForm from '@/components/election/cumulative/CumulativeVotingForm.vue';
import AntiPluralityVotingForm from '../election/anti_plurality/AntiPluralityVotingForm.vue';
import SinglePartyVotingForm from '../election/single_party/SinglePartyVotingForm.vue';
import ThreeTwoOneVotingForm from '../election/three_two_one/ThreeTwoOneVotingForm.vue';
import CondorcetMethodVotingForm from '../election/condorcet_method/CondorcetMethodVotingForm.vue';
import MajorityJudgmentVotingForm from '../election/majority_judgment/MajorityJudgmentVotingForm.vue';
import ScoreVotingForm from '../election/score/ScoreVotingForm.vue';
import UsualJudgmentVotingForm from '../election/usual_judgment/UsualJudgmentVotingForm.vue';
import SingleNonTransferableVotingForm from '../election/single_non_transferable/SingleNonTransferableVotingForm.vue';
import InfoCom from '../InfoCom.vue';
import VoteInfo from '../VoteInfo.vue';

const props = defineProps({
    election_id: {
        type: String,
        required: true,
    },
    vote_token: {
        type: String,
        required: false,
    },
    election_type: {
        type: Object as PropType<ElectionType>,
        required: true,
    }
});


const loading = ref<boolean>(true);
const election = ref<GenericElection | CumulativeElection | null>(null);
const options = ref<VoteOption[]>([]);
const error = ref<String | null>(null)
const complete = ref(false);

onMounted(async () => {
    console.log(`getting election ${props.election_type}`);
    election.value = await get_generic_election(props.election_type, props.election_id);
    options.value = options_to_vote_options(election.value.options);
    loading.value = false;
    console.log(`election gotten ${JSON.stringify(election.value)}`);
    if (election.value.require_token && !props.vote_token) {
        error.value = "This election requires a token to vote";
        return;
    }
});

function on_complete() {
    complete.value = true;
}

function on_submit_error(message: string) {
    error.value = message;
}

const thankyou_sentences: string[] = [
    "Living democracy, thanks! 🗳️",
    "Grateful for democracy 🙏",
    "Appreciating democratic commitment 🌟",
    "Dedication admired 👏",
    "Democratic engagement cheers 🎉",
    "Active democracy, thanks 🤝",
    "Kudos for democratic values 🏆",
    "Participation appreciated 🎖️",
    "Promoting democracy 📢",
    "Democratic spirit gratitude 💙",
    "Dedication appreciated 🌐",
    "You help democracy thrive 🌱",
    "Thanks for democratic actions 💌",
    "Support means a lot 💪",
    "SO committed, huh? 🙄",
    "Involvement celebrated 🎊",
    "Contributions valued 💎",
    "Inspiring democratic passion 🌟",
    "Democracy stronger 💪",
    "Beacon of hope? 😏",
    "Actively contributing 🗳️",
    "Democracy's savior? 🤨",
    "World changer 😒",
    "One vote hero 🙄",
    "Fighting like a hero 😒",
    "Thank you so very much 😊 for your undeniably extraordinary, remarkably exceptional, and genuinely awe-inspiring commitment to the grand, important, and all-encompassing concept of democracy 🗳️, a system of governance that holds the power of the people 👥 at its very core, and for which you have demonstrated an unwavering, steadfast, and resolute passion 🌟, a passion that has undoubtedly had a significant, meaningful, and lasting impact on not only your own community 🏘️, but also on the broader society as a whole, for your actions, whether big or small, grand or modest, have reverberated throughout the social fabric, echoing the values and principles of democracy, such as freedom 🕊️, equality ⚖️, and justice, values and principles that are so essential, so necessary, and so vital to the well-being, happiness 😄, and prosperity of our collective society, and it is because of individuals like you, individuals who possess a burning desire 🔥, a deep-seated yearning, and an unquenchable thirst for the betterment of humanity, that our democracy is able to flourish, to grow 🌱, and to thrive, evolving and adapting to the ever-changing landscape of our world 🌍, a world that is in desperate need of heroes, of role models, and of champions of democracy, and it is in this spirit that we wholeheartedly, sincerely, and unreservedly extend our utmost gratitude 🙏, our deepest appreciation, and our most profound thanks for your invaluable, irreplaceable, and truly incomparable contributions to the democratic process, a process that is made richer, stronger, and more vibrant by your very presence, your tireless efforts, and your unwavering determination to make the world a better place for all 💖.",
];

function random_thank_you(): string {
    const random_index = Math.floor(Math.random() * thankyou_sentences.length);
    return thankyou_sentences[random_index];
}

const show_info = ref(false);

</script>

<template>
    <div>
        <div v-if="complete">
            <p>{{ random_thank_you() }}</p>
        </div>
        <div v-else-if="election">
            <button @click="show_info = !show_info">{{ !show_info ? `Click here to read about ${election_type} Elections` :
                `Hide` }}
            </button>
            <div v-if="show_info">
                <h1>Info</h1>
                <InfoCom :election_type="election_type" />
                <br />
                <hr />
            </div>
            <h1>Voting Form</h1>
            <VoteInfo :election_type="election_type" />
            <p v-if="error" class="error-text">
                Error: {{ error }}
            </p>
            <br />
            <h2>Title: {{ election.title }}</h2>
            <PrefElectionVotingForm v-if="election_type == ElectionType.PreferentialVoting" :vote_token="vote_token"
                :election="election" :options="options" @complete="on_complete" @error="on_submit_error" />
            <SvtVotingForm v-else-if="election_type == ElectionType.SingleTransferableVote" :vote_token="vote_token"
                :election="election" :options="options" @complete="on_complete" @error="on_submit_error" />
            <BordaCountVotingForm v-else-if="election_type == ElectionType.BordaCount" :vote_token="vote_token"
                :election="election" :options="options" @complete="on_complete" @error="on_submit_error" />
            <ApprovalVotingForm v-else-if="election_type == ElectionType.Approval" :vote_token="vote_token"
                :election="election" :options="options" @complete="on_complete" @error="on_submit_error" />
            <StarVotingForm v-else-if="election_type == ElectionType.Star" :vote_token="vote_token" :election="election"
                :options="options" @complete="on_complete" @error="on_submit_error" />
            <CumulativeVotingForm v-else-if="election_type == ElectionType.Cumulative" :vote_token="vote_token"
                :election="(election as CumulativeElection)" :options="options" @complete="on_complete"
                @error="on_submit_error" />
            <AntiPluralityVotingForm v-else-if="election_type == ElectionType.AntiPlurality" :vote_token="vote_token"
                :election="election" :options="options" @complete="on_complete" @error="on_submit_error" />
            <SinglePartyVotingForm v-else-if="election_type == ElectionType.SingleParty" :vote_token="vote_token"
                :election="election" :options="options" @complete="on_complete" @error="on_submit_error" />
            <ThreeTwoOneVotingForm v-else-if="election_type == ElectionType.ThreeTwoOne" :vote_token="vote_token"
                :election="election" :options="options" @complete="on_complete" @error="on_submit_error" />
            <CondorcetMethodVotingForm v-else-if="election_type == ElectionType.CondorcetMethod" :vote_token="vote_token"
                :election="election" :options="options" @complete="on_complete" @error="on_submit_error" />
            <MajorityJudgmentVotingForm v-else-if="election_type == ElectionType.MajorityJudgment" :vote_token="vote_token"
                :election="election" :options="options" @complete="on_complete" @error="on_submit_error" />
            <ScoreVotingForm v-else-if="election_type == ElectionType.Score" :vote_token="vote_token"
                :election="(election as ScoreElection)" :options="options" @complete="on_complete"
                @error="on_submit_error" />
            <UsualJudgmentVotingForm v-else-if="election_type == ElectionType.UsualJudgment" :vote_token="vote_token"
                :election="election" :options="options" @complete="on_complete" @error="on_submit_error" />
            <SingleNonTransferableVotingForm v-else-if="election_type == ElectionType.SingleNonTransferable"
                :vote_token="vote_token" :election="election" :options="options" @complete="on_complete"
                @error="on_submit_error" />
            <p v-else>Unimplemented</p>
        </div>
    </div>
</template>

<style scoped></style>
