# MBTX execution comparison

```json
{
  "boundary_observations": [
    {
      "confidence": "inferred",
      "layer": "sandbox",
      "message": "A separate PID namespace is observed. Sandbox teardown may terminate the descendant before its tail write; emitted tail bytes must still be present in Codex output."
    },
    {
      "confidence": "unknown",
      "layer": "sandbox",
      "message": "The sandbox outer wait normalizes signal termination to an exit code. This Codex sample does not prove inner Shell signal identity; native launcher contracts check real wait status separately."
    }
  ],
  "default_shell_is_direct": true,
  "intention_to_treat": {
    "arm_statuses": {
      "success": 480
    },
    "attempted_pairs": 240
  },
  "limits": [
    "No universal lossless replacement claim from this sample.",
    "Model compute and relay internal waiting are inseparable without server evidence.",
    "Intervals overlap; local residual and missing stages remain unexplained.",
    "Online scenario samples do not support stable p99 estimates.",
    "Default Shell and Direct control runs are separate artifacts; inspect observed shell_modes before attributing differences."
  ],
  "partial": false,
  "platform": "linux",
  "purpose": "formal",
  "stop": {
    "attempted_pairs": 240,
    "elapsed_seconds": 5167.503367132,
    "failed_arms": 0,
    "partial": false,
    "schema_version": 2,
    "stop_reason": null,
    "target_pairs": 240,
    "valid_pairs": 240
  },
  "strict_comparable_pairs": 240,
  "target_pairs": 240
}
```

| Stratum | Metric | Pairs | MBTX/Shell | 95% interval |
|---|---|---:|---:|---|
| linux / argv_empty_unicode / round 1 / code / shell ["Direct"] | approval_ns | 10 | 0.958682198762286 | [0.773297861349313,1.1701125271149675] |
| linux / argv_empty_unicode / round 1 / code / shell ["Direct"] | artifact_write_ns | 10 | 0.3638302831360922 | [0.15253450217696052,1.1053244467781558] |
| linux / argv_empty_unicode / round 1 / code / shell ["Direct"] | codex_total_ns | 10 | 1.000218181206492 | [0.9994930493477776,1.0007943563560162] |
| linux / argv_empty_unicode / round 1 / code / shell ["Direct"] | external_response_ns | 0 | null | null |
| linux / argv_empty_unicode / round 1 / code / shell ["Direct"] | fixture_runtime_ns | 10 | 1.0297525060026618 | [0.98153469396633,1.0766600958524373] |
| linux / argv_empty_unicode / round 1 / code / shell ["Direct"] | launcher_child_spawn_ns | 0 | null | null |
| linux / argv_empty_unicode / round 1 / code / shell ["Direct"] | launcher_prepare_ns | 0 | null | null |
| linux / argv_empty_unicode / round 1 / code / shell ["Direct"] | policy_sandbox_spawn_ns | 10 | 1.025936619346561 | [0.994561123167468,1.0595179986541334] |
| linux / argv_empty_unicode / round 1 / code / shell ["Direct"] | rate_wait_ns | 0 | null | null |
| linux / argv_empty_unicode / round 1 / code / shell ["Direct"] | ready_to_wait_observed_ns | 10 | 1.070817252360861 | [1.0431433584140122,1.099123716936576] |
| linux / argv_empty_unicode / round 1 / code / shell ["Direct"] | request_gate_queue_ns | 0 | null | null |
| linux / argv_empty_unicode / round 1 / code / shell ["Direct"] | request_to_first_byte_ns | 0 | null | null |
| linux / argv_empty_unicode / round 1 / code / shell ["Direct"] | sandbox_setup_spawn_ns | 10 | 1.0263119049318252 | [0.9949879761358936,1.0598112063967111] |
| linux / argv_empty_unicode / round 1 / code / shell ["Direct"] | spawn_return_ns | 10 | 0.9614988591652628 | [0.8752793240799697,1.060180520920555] |
| linux / argv_empty_unicode / round 1 / code / shell ["Direct"] | startup_to_ready_ns | 10 | 1.0431442061615923 | [0.9791695416907664,1.1171310486385786] |
| linux / argv_empty_unicode / round 1 / code / shell ["Direct"] | tool_call_ns | 10 | 1.0246503367499558 | [0.9874259006635924,1.061247489698101] |
| linux / argv_empty_unicode / round 1 / code / shell ["Direct"] | unexplained_ns | 10 | 1.0000713774439278 | [0.9994701215937588,1.00052992152348] |
| linux / argv_empty_unicode / round 1 / code / shell ["Direct"] | wait_to_last_observed_eof_ns | 0 | null | null |
| linux / argv_long / round 1 / code / shell ["Direct"] | approval_ns | 10 | 0.7105724299065421 | [0.548761705818832,0.9625259177948532] |
| linux / argv_long / round 1 / code / shell ["Direct"] | artifact_write_ns | 10 | 1.1114517282633316 | [0.6415309399350867,1.7895191161848207] |
| linux / argv_long / round 1 / code / shell ["Direct"] | codex_total_ns | 10 | 1.10702602268118 | [0.9986209040727476,1.4110089981260547] |
| linux / argv_long / round 1 / code / shell ["Direct"] | external_response_ns | 0 | null | null |
| linux / argv_long / round 1 / code / shell ["Direct"] | fixture_runtime_ns | 10 | 1.0296186130756309 | [0.9252681737762856,1.1756407242596116] |
| linux / argv_long / round 1 / code / shell ["Direct"] | launcher_child_spawn_ns | 0 | null | null |
| linux / argv_long / round 1 / code / shell ["Direct"] | launcher_prepare_ns | 0 | null | null |
| linux / argv_long / round 1 / code / shell ["Direct"] | policy_sandbox_spawn_ns | 10 | 1.0302529912744955 | [1.0082693988475753,1.0445347876123996] |
| linux / argv_long / round 1 / code / shell ["Direct"] | rate_wait_ns | 0 | null | null |
| linux / argv_long / round 1 / code / shell ["Direct"] | ready_to_wait_observed_ns | 10 | 1.0568784456181906 | [0.9836955315785748,1.143956528013419] |
| linux / argv_long / round 1 / code / shell ["Direct"] | request_gate_queue_ns | 0 | null | null |
| linux / argv_long / round 1 / code / shell ["Direct"] | request_to_first_byte_ns | 0 | null | null |
| linux / argv_long / round 1 / code / shell ["Direct"] | sandbox_setup_spawn_ns | 10 | 1.031507339272357 | [1.0096827055138735,1.045480592501211] |
| linux / argv_long / round 1 / code / shell ["Direct"] | spawn_return_ns | 10 | 0.9608637261448644 | [0.881496595844473,1.06122226685291] |
| linux / argv_long / round 1 / code / shell ["Direct"] | startup_to_ready_ns | 10 | 1.05909470371372 | [1.0153022385304844,1.096797724956962] |
| linux / argv_long / round 1 / code / shell ["Direct"] | tool_call_ns | 10 | 1.0100347642604268 | [0.9767745627724288,1.0405261401249564] |
| linux / argv_long / round 1 / code / shell ["Direct"] | unexplained_ns | 10 | 1.1076380329127036 | [0.9987023736784292,1.4141668139314885] |
| linux / argv_long / round 1 / code / shell ["Direct"] | wait_to_last_observed_eof_ns | 0 | null | null |
| linux / argv_metacharacters / round 1 / code / shell ["Direct"] | approval_ns | 10 | 0.96675008938148 | [0.7578029921201765,1.229079379458725] |
| linux / argv_metacharacters / round 1 / code / shell ["Direct"] | artifact_write_ns | 10 | 1.127568504958336 | [0.30724439470349196,3.194228002406678] |
| linux / argv_metacharacters / round 1 / code / shell ["Direct"] | codex_total_ns | 10 | 1.0007241374272655 | [1.0002439144121584,1.0015163577377515] |
| linux / argv_metacharacters / round 1 / code / shell ["Direct"] | external_response_ns | 0 | null | null |
| linux / argv_metacharacters / round 1 / code / shell ["Direct"] | fixture_runtime_ns | 10 | 0.9770454724518663 | [0.9384145190665728,1.0144379136138644] |
| linux / argv_metacharacters / round 1 / code / shell ["Direct"] | launcher_child_spawn_ns | 0 | null | null |
| linux / argv_metacharacters / round 1 / code / shell ["Direct"] | launcher_prepare_ns | 0 | null | null |
| linux / argv_metacharacters / round 1 / code / shell ["Direct"] | policy_sandbox_spawn_ns | 10 | 1.0157783395838251 | [0.9952165218287302,1.038988767171713] |
| linux / argv_metacharacters / round 1 / code / shell ["Direct"] | rate_wait_ns | 0 | null | null |
| linux / argv_metacharacters / round 1 / code / shell ["Direct"] | ready_to_wait_observed_ns | 10 | 1.0172240403316128 | [0.9794566972060352,1.059432231541584] |
| linux / argv_metacharacters / round 1 / code / shell ["Direct"] | request_gate_queue_ns | 0 | null | null |
| linux / argv_metacharacters / round 1 / code / shell ["Direct"] | request_to_first_byte_ns | 0 | null | null |
| linux / argv_metacharacters / round 1 / code / shell ["Direct"] | sandbox_setup_spawn_ns | 10 | 1.0155051822362695 | [0.9948776583398272,1.0388957737574254] |
| linux / argv_metacharacters / round 1 / code / shell ["Direct"] | spawn_return_ns | 10 | 0.9802389197331708 | [0.9018630376542828,1.0658305072942504] |
| linux / argv_metacharacters / round 1 / code / shell ["Direct"] | startup_to_ready_ns | 10 | 1.029123604549196 | [0.9876546271010704,1.0759809011832655] |
| linux / argv_metacharacters / round 1 / code / shell ["Direct"] | tool_call_ns | 10 | 1.0261539932296355 | [0.9935946443085456,1.063806176004388] |
| linux / argv_metacharacters / round 1 / code / shell ["Direct"] | unexplained_ns | 10 | 1.0005879547199117 | [1.000135433990038,1.0012655228834015] |
| linux / argv_metacharacters / round 1 / code / shell ["Direct"] | wait_to_last_observed_eof_ns | 0 | null | null |
| linux / cancel_ctrl_c / round 1 / code / shell ["Direct"] | approval_ns | 10 | 0.9497341148158212 | [0.8670162059103909,1.012530722430396] |
| linux / cancel_ctrl_c / round 1 / code / shell ["Direct"] | artifact_write_ns | 10 | 0.734691453688801 | [0.1485061090176752,2.641976736961845] |
| linux / cancel_ctrl_c / round 1 / code / shell ["Direct"] | codex_total_ns | 10 | 1.0945507213172103 | [0.9984396704379104,1.3522169402623825] |
| linux / cancel_ctrl_c / round 1 / code / shell ["Direct"] | external_response_ns | 0 | null | null |
| linux / cancel_ctrl_c / round 1 / code / shell ["Direct"] | fixture_runtime_ns | 0 | null | null |
| linux / cancel_ctrl_c / round 1 / code / shell ["Direct"] | launcher_child_spawn_ns | 0 | null | null |
| linux / cancel_ctrl_c / round 1 / code / shell ["Direct"] | launcher_prepare_ns | 0 | null | null |
| linux / cancel_ctrl_c / round 1 / code / shell ["Direct"] | policy_sandbox_spawn_ns | 10 | 1.000691919477515 | [0.9982965095927708,1.003494181763084] |
| linux / cancel_ctrl_c / round 1 / code / shell ["Direct"] | rate_wait_ns | 0 | null | null |
| linux / cancel_ctrl_c / round 1 / code / shell ["Direct"] | ready_to_wait_observed_ns | 10 | 0.9987719006822872 | [0.9979937899834418,0.9996364257403704] |
| linux / cancel_ctrl_c / round 1 / code / shell ["Direct"] | request_gate_queue_ns | 0 | null | null |
| linux / cancel_ctrl_c / round 1 / code / shell ["Direct"] | request_to_first_byte_ns | 0 | null | null |
| linux / cancel_ctrl_c / round 1 / code / shell ["Direct"] | sandbox_setup_spawn_ns | 10 | 1.0007570316548855 | [0.998351444436922,1.003528231907744] |
| linux / cancel_ctrl_c / round 1 / code / shell ["Direct"] | spawn_return_ns | 10 | 1.0358593304305657 | [0.9921297765543228,1.083481972098572] |
| linux / cancel_ctrl_c / round 1 / code / shell ["Direct"] | startup_to_ready_ns | 10 | 1.07655072150991 | [1.0237516514359162,1.128007339570684] |
| linux / cancel_ctrl_c / round 1 / code / shell ["Direct"] | tool_call_ns | 10 | 0.999812773099312 | [0.999060122953188,1.0006040625332] |
| linux / cancel_ctrl_c / round 1 / code / shell ["Direct"] | unexplained_ns | 10 | 1.1074874239850103 | [0.9982806427851084,1.413643639799241] |
| linux / cancel_ctrl_c / round 1 / code / shell ["Direct"] | wait_to_last_observed_eof_ns | 0 | null | null |
| linux / cancel_sigkill / round 1 / code / shell ["Direct"] | approval_ns | 10 | 1.0501899936668777 | [0.8394253188862981,1.306975963756869] |
| linux / cancel_sigkill / round 1 / code / shell ["Direct"] | artifact_write_ns | 10 | 0.7500222354872784 | [0.404153550616546,1.64847105846995] |
| linux / cancel_sigkill / round 1 / code / shell ["Direct"] | codex_total_ns | 10 | 0.9999254994002016 | [0.9995069034266958,1.000325428045207] |
| linux / cancel_sigkill / round 1 / code / shell ["Direct"] | external_response_ns | 0 | null | null |
| linux / cancel_sigkill / round 1 / code / shell ["Direct"] | fixture_runtime_ns | 0 | null | null |
| linux / cancel_sigkill / round 1 / code / shell ["Direct"] | launcher_child_spawn_ns | 0 | null | null |
| linux / cancel_sigkill / round 1 / code / shell ["Direct"] | launcher_prepare_ns | 0 | null | null |
| linux / cancel_sigkill / round 1 / code / shell ["Direct"] | policy_sandbox_spawn_ns | 10 | 1.0019455629038518 | [1.0004436296906123,1.0035704781890384] |
| linux / cancel_sigkill / round 1 / code / shell ["Direct"] | rate_wait_ns | 0 | null | null |
| linux / cancel_sigkill / round 1 / code / shell ["Direct"] | ready_to_wait_observed_ns | 10 | 0.9967941679874858 | [0.9954872485899744,0.9981567360960644] |
| linux / cancel_sigkill / round 1 / code / shell ["Direct"] | request_gate_queue_ns | 0 | null | null |
| linux / cancel_sigkill / round 1 / code / shell ["Direct"] | request_to_first_byte_ns | 0 | null | null |
| linux / cancel_sigkill / round 1 / code / shell ["Direct"] | sandbox_setup_spawn_ns | 10 | 1.0019321296628994 | [1.0004501781141624,1.0035521683225466] |
| linux / cancel_sigkill / round 1 / code / shell ["Direct"] | spawn_return_ns | 10 | 0.9952902520425276 | [0.9195858871784196,1.0716944770889305] |
| linux / cancel_sigkill / round 1 / code / shell ["Direct"] | startup_to_ready_ns | 10 | 1.0323513423218076 | [0.9884924036295196,1.078967368850391] |
| linux / cancel_sigkill / round 1 / code / shell ["Direct"] | tool_call_ns | 10 | 0.9974653540045882 | [0.9961830276955124,0.9988923248244695] |
| linux / cancel_sigkill / round 1 / code / shell ["Direct"] | unexplained_ns | 10 | 1.0003572875361166 | [0.9998066351605024,1.0009144437433433] |
| linux / cancel_sigkill / round 1 / code / shell ["Direct"] | wait_to_last_observed_eof_ns | 6 | 1.2711462089743004 | [0.9484990559093672,1.8516984939646477] |
| linux / cancel_sigterm / round 1 / code / shell ["Direct"] | approval_ns | 10 | 0.9370815313150728 | [0.735650608847268,1.195713520271477] |
| linux / cancel_sigterm / round 1 / code / shell ["Direct"] | artifact_write_ns | 10 | 0.7496597563596235 | [0.21134717539606587,4.356308120173036] |
| linux / cancel_sigterm / round 1 / code / shell ["Direct"] | codex_total_ns | 10 | 1.0007945470099755 | [0.9992128962310952,1.002588639408275] |
| linux / cancel_sigterm / round 1 / code / shell ["Direct"] | external_response_ns | 0 | null | null |
| linux / cancel_sigterm / round 1 / code / shell ["Direct"] | fixture_runtime_ns | 0 | null | null |
| linux / cancel_sigterm / round 1 / code / shell ["Direct"] | launcher_child_spawn_ns | 0 | null | null |
| linux / cancel_sigterm / round 1 / code / shell ["Direct"] | launcher_prepare_ns | 0 | null | null |
| linux / cancel_sigterm / round 1 / code / shell ["Direct"] | policy_sandbox_spawn_ns | 10 | 0.9991872906770424 | [0.9964568038356908,1.0016298753341202] |
| linux / cancel_sigterm / round 1 / code / shell ["Direct"] | rate_wait_ns | 0 | null | null |
| linux / cancel_sigterm / round 1 / code / shell ["Direct"] | ready_to_wait_observed_ns | 10 | 0.9985699641969072 | [0.9958113809112752,1.001419602021676] |
| linux / cancel_sigterm / round 1 / code / shell ["Direct"] | request_gate_queue_ns | 0 | null | null |
| linux / cancel_sigterm / round 1 / code / shell ["Direct"] | request_to_first_byte_ns | 0 | null | null |
| linux / cancel_sigterm / round 1 / code / shell ["Direct"] | sandbox_setup_spawn_ns | 10 | 0.9992402402843044 | [0.9965657498834524,1.001670415468701] |
| linux / cancel_sigterm / round 1 / code / shell ["Direct"] | spawn_return_ns | 10 | 0.9938060189485484 | [0.8863495629481165,1.1143511623275155] |
| linux / cancel_sigterm / round 1 / code / shell ["Direct"] | startup_to_ready_ns | 10 | 1.072015779975273 | [1.0302537401179082,1.117334995443311] |
| linux / cancel_sigterm / round 1 / code / shell ["Direct"] | tool_call_ns | 10 | 0.9992024138525984 | [0.9961092684741282,1.0028192851991609] |
| linux / cancel_sigterm / round 1 / code / shell ["Direct"] | unexplained_ns | 10 | 1.0009952326753395 | [0.999459609523788,1.0028069549227916] |
| linux / cancel_sigterm / round 1 / code / shell ["Direct"] | wait_to_last_observed_eof_ns | 2 | 0.970956686512346 | [0.9411498880512704,1.0103776147785928] |
| linux / cwd_symlink / round 1 / code / shell ["Direct"] | approval_ns | 10 | 1.277993684721885 | [1.0484238660482663,1.5318204242723237] |
| linux / cwd_symlink / round 1 / code / shell ["Direct"] | artifact_write_ns | 10 | 1.5273771732856212 | [0.6768790536155632,5.576374209769701] |
| linux / cwd_symlink / round 1 / code / shell ["Direct"] | codex_total_ns | 10 | 0.9999779895263552 | [0.9995583852702032,1.0004163378989182] |
| linux / cwd_symlink / round 1 / code / shell ["Direct"] | external_response_ns | 0 | null | null |
| linux / cwd_symlink / round 1 / code / shell ["Direct"] | fixture_runtime_ns | 10 | 0.986741880406108 | [0.8903487986104451,1.1146194270292904] |
| linux / cwd_symlink / round 1 / code / shell ["Direct"] | launcher_child_spawn_ns | 0 | null | null |
| linux / cwd_symlink / round 1 / code / shell ["Direct"] | launcher_prepare_ns | 0 | null | null |
| linux / cwd_symlink / round 1 / code / shell ["Direct"] | policy_sandbox_spawn_ns | 10 | 1.0183360996288762 | [0.995037226093176,1.0403671993294703] |
| linux / cwd_symlink / round 1 / code / shell ["Direct"] | rate_wait_ns | 0 | null | null |
| linux / cwd_symlink / round 1 / code / shell ["Direct"] | ready_to_wait_observed_ns | 10 | 1.00233042494305 | [0.9569691679422566,1.0655143834033007] |
| linux / cwd_symlink / round 1 / code / shell ["Direct"] | request_gate_queue_ns | 0 | null | null |
| linux / cwd_symlink / round 1 / code / shell ["Direct"] | request_to_first_byte_ns | 0 | null | null |
| linux / cwd_symlink / round 1 / code / shell ["Direct"] | sandbox_setup_spawn_ns | 10 | 1.01808467925988 | [0.9946624104128928,1.0405723185320457] |
| linux / cwd_symlink / round 1 / code / shell ["Direct"] | spawn_return_ns | 10 | 1.0886290420422036 | [1.008137737231692,1.1842638774863132] |
| linux / cwd_symlink / round 1 / code / shell ["Direct"] | startup_to_ready_ns | 10 | 1.0386276947763229 | [0.9928428942534322,1.083469993630081] |
| linux / cwd_symlink / round 1 / code / shell ["Direct"] | tool_call_ns | 10 | 1.029626937918765 | [1.0051974379910658,1.0554337129333242] |
| linux / cwd_symlink / round 1 / code / shell ["Direct"] | unexplained_ns | 10 | 0.9998007873422816 | [0.9994261315299297,1.0001723957072894] |
| linux / cwd_symlink / round 1 / code / shell ["Direct"] | wait_to_last_observed_eof_ns | 0 | null | null |
| linux / descendant_cancel / round 1 / code / shell ["Direct"] | approval_ns | 10 | 0.954197099989206 | [0.7834197233319027,1.2086919603226678] |
| linux / descendant_cancel / round 1 / code / shell ["Direct"] | artifact_write_ns | 10 | 0.8698532360960517 | [0.43851038675843906,1.6474553117910282] |
| linux / descendant_cancel / round 1 / code / shell ["Direct"] | codex_total_ns | 10 | 0.99986183329539 | [0.9986703783162078,1.0011205958396496] |
| linux / descendant_cancel / round 1 / code / shell ["Direct"] | external_response_ns | 0 | null | null |
| linux / descendant_cancel / round 1 / code / shell ["Direct"] | fixture_runtime_ns | 0 | null | null |
| linux / descendant_cancel / round 1 / code / shell ["Direct"] | launcher_child_spawn_ns | 0 | null | null |
| linux / descendant_cancel / round 1 / code / shell ["Direct"] | launcher_prepare_ns | 0 | null | null |
| linux / descendant_cancel / round 1 / code / shell ["Direct"] | policy_sandbox_spawn_ns | 10 | 1.0002198686807071 | [0.9987109631312856,1.0016498890455572] |
| linux / descendant_cancel / round 1 / code / shell ["Direct"] | rate_wait_ns | 0 | null | null |
| linux / descendant_cancel / round 1 / code / shell ["Direct"] | ready_to_wait_observed_ns | 10 | 0.9989501040992614 | [0.9978093775866892,1.0002074928548308] |
| linux / descendant_cancel / round 1 / code / shell ["Direct"] | request_gate_queue_ns | 0 | null | null |
| linux / descendant_cancel / round 1 / code / shell ["Direct"] | request_to_first_byte_ns | 0 | null | null |
| linux / descendant_cancel / round 1 / code / shell ["Direct"] | sandbox_setup_spawn_ns | 10 | 1.0001540875600166 | [0.9986666099586944,1.0015662881071967] |
| linux / descendant_cancel / round 1 / code / shell ["Direct"] | spawn_return_ns | 10 | 0.972574734268992 | [0.914759258307002,1.0219951612042464] |
| linux / descendant_cancel / round 1 / code / shell ["Direct"] | startup_to_ready_ns | 10 | 1.0729382838483557 | [1.0101419021226732,1.1339775435044814] |
| linux / descendant_cancel / round 1 / code / shell ["Direct"] | tool_call_ns | 10 | 1.000700689927083 | [0.999520632464448,1.0016925163631472] |
| linux / descendant_cancel / round 1 / code / shell ["Direct"] | unexplained_ns | 10 | 0.9997666029618412 | [0.9983412701644684,1.001257337735702] |
| linux / descendant_cancel / round 1 / code / shell ["Direct"] | wait_to_last_observed_eof_ns | 1 | 1.0307932176963557 | [1.0307932176963557,1.0307932176963557] |
| linux / descendant_pipe_tail / round 1 / code / shell ["Direct"] | approval_ns | 10 | 0.8130474609263182 | [0.5748841251448435,1.108468636006008] |
| linux / descendant_pipe_tail / round 1 / code / shell ["Direct"] | artifact_write_ns | 10 | 0.6453081380515522 | [0.3089663712686555,1.3852553488987691] |
| linux / descendant_pipe_tail / round 1 / code / shell ["Direct"] | codex_total_ns | 10 | 1.0004290057695842 | [0.9996865028458534,1.0012169224777956] |
| linux / descendant_pipe_tail / round 1 / code / shell ["Direct"] | external_response_ns | 0 | null | null |
| linux / descendant_pipe_tail / round 1 / code / shell ["Direct"] | fixture_runtime_ns | 10 | 0.89089078738188 | [0.8013557101063364,0.9900637567698396] |
| linux / descendant_pipe_tail / round 1 / code / shell ["Direct"] | launcher_child_spawn_ns | 0 | null | null |
| linux / descendant_pipe_tail / round 1 / code / shell ["Direct"] | launcher_prepare_ns | 0 | null | null |
| linux / descendant_pipe_tail / round 1 / code / shell ["Direct"] | policy_sandbox_spawn_ns | 10 | 1.0431792904488584 | [1.0136106867388035,1.0791400881752904] |
| linux / descendant_pipe_tail / round 1 / code / shell ["Direct"] | rate_wait_ns | 0 | null | null |
| linux / descendant_pipe_tail / round 1 / code / shell ["Direct"] | ready_to_wait_observed_ns | 10 | 1.0627348727931345 | [0.9037022223456866,1.3324318051545532] |
| linux / descendant_pipe_tail / round 1 / code / shell ["Direct"] | request_gate_queue_ns | 0 | null | null |
| linux / descendant_pipe_tail / round 1 / code / shell ["Direct"] | request_to_first_byte_ns | 0 | null | null |
| linux / descendant_pipe_tail / round 1 / code / shell ["Direct"] | sandbox_setup_spawn_ns | 10 | 1.0435953251142642 | [1.0138876442678697,1.0797695069631197] |
| linux / descendant_pipe_tail / round 1 / code / shell ["Direct"] | spawn_return_ns | 10 | 0.9593951038925572 | [0.8612967615761945,1.067949306830202] |
| linux / descendant_pipe_tail / round 1 / code / shell ["Direct"] | startup_to_ready_ns | 10 | 1.0850982876952446 | [1.042318812507315,1.133305856470468] |
| linux / descendant_pipe_tail / round 1 / code / shell ["Direct"] | tool_call_ns | 10 | 1.0273966357267257 | [0.9939998982229644,1.0585438805287533] |
| linux / descendant_pipe_tail / round 1 / code / shell ["Direct"] | unexplained_ns | 10 | 1.0002618928163467 | [0.9996641352763904,1.000928732060428] |
| linux / descendant_pipe_tail / round 1 / code / shell ["Direct"] | wait_to_last_observed_eof_ns | 0 | null | null |
| linux / environment_empty_unset / round 1 / code / shell ["Direct"] | approval_ns | 10 | 0.9816476892163428 | [0.7541078265235674,1.2671745698147905] |
| linux / environment_empty_unset / round 1 / code / shell ["Direct"] | artifact_write_ns | 10 | 1.012270924260595 | [0.44765794329345543,1.774798429812504] |
| linux / environment_empty_unset / round 1 / code / shell ["Direct"] | codex_total_ns | 10 | 1.000067890617427 | [0.999091576320323,1.0008721462548034] |
| linux / environment_empty_unset / round 1 / code / shell ["Direct"] | external_response_ns | 0 | null | null |
| linux / environment_empty_unset / round 1 / code / shell ["Direct"] | fixture_runtime_ns | 10 | 0.565041556153033 | [0.29394575756966984,1.0630764826088388] |
| linux / environment_empty_unset / round 1 / code / shell ["Direct"] | launcher_child_spawn_ns | 0 | null | null |
| linux / environment_empty_unset / round 1 / code / shell ["Direct"] | launcher_prepare_ns | 0 | null | null |
| linux / environment_empty_unset / round 1 / code / shell ["Direct"] | policy_sandbox_spawn_ns | 10 | 0.9748345732220758 | [0.9072706233765228,1.0203872942913192] |
| linux / environment_empty_unset / round 1 / code / shell ["Direct"] | rate_wait_ns | 0 | null | null |
| linux / environment_empty_unset / round 1 / code / shell ["Direct"] | ready_to_wait_observed_ns | 10 | 0.7915018907704126 | [0.5219127172229884,1.0924747581518492] |
| linux / environment_empty_unset / round 1 / code / shell ["Direct"] | request_gate_queue_ns | 0 | null | null |
| linux / environment_empty_unset / round 1 / code / shell ["Direct"] | request_to_first_byte_ns | 0 | null | null |
| linux / environment_empty_unset / round 1 / code / shell ["Direct"] | sandbox_setup_spawn_ns | 10 | 0.9747876143508152 | [0.9070942429174528,1.020384575911517] |
| linux / environment_empty_unset / round 1 / code / shell ["Direct"] | spawn_return_ns | 10 | 1.000053171169687 | [0.902785165580723,1.0956306560835505] |
| linux / environment_empty_unset / round 1 / code / shell ["Direct"] | startup_to_ready_ns | 10 | 1.0075811300047617 | [0.9766329666131917,1.039339933679895] |
| linux / environment_empty_unset / round 1 / code / shell ["Direct"] | tool_call_ns | 10 | 0.9644153204284835 | [0.8949279149782601,1.0219036206922347] |
| linux / environment_empty_unset / round 1 / code / shell ["Direct"] | unexplained_ns | 10 | 1.000271274376747 | [0.99966286316894,1.0008007664566072] |
| linux / environment_empty_unset / round 1 / code / shell ["Direct"] | wait_to_last_observed_eof_ns | 0 | null | null |
| linux / environment_path_login / round 1 / code / shell ["Direct"] | approval_ns | 10 | 0.9518915060670948 | [0.6555297434157291,1.3233934695094678] |
| linux / environment_path_login / round 1 / code / shell ["Direct"] | artifact_write_ns | 10 | 1.014618881408505 | [0.33678712275839645,2.8838909514833233] |
| linux / environment_path_login / round 1 / code / shell ["Direct"] | codex_total_ns | 10 | 1.000138629459918 | [0.999264044604945,1.000958983939484] |
| linux / environment_path_login / round 1 / code / shell ["Direct"] | external_response_ns | 0 | null | null |
| linux / environment_path_login / round 1 / code / shell ["Direct"] | fixture_runtime_ns | 10 | 1.0335918542245088 | [0.8439685901165093,1.227827279040909] |
| linux / environment_path_login / round 1 / code / shell ["Direct"] | launcher_child_spawn_ns | 0 | null | null |
| linux / environment_path_login / round 1 / code / shell ["Direct"] | launcher_prepare_ns | 0 | null | null |
| linux / environment_path_login / round 1 / code / shell ["Direct"] | policy_sandbox_spawn_ns | 10 | 1.01951878676678 | [0.9899048542620064,1.0527177971278] |
| linux / environment_path_login / round 1 / code / shell ["Direct"] | rate_wait_ns | 0 | null | null |
| linux / environment_path_login / round 1 / code / shell ["Direct"] | ready_to_wait_observed_ns | 10 | 1.0140634774795183 | [0.9283942330073156,1.0957229393588352] |
| linux / environment_path_login / round 1 / code / shell ["Direct"] | request_gate_queue_ns | 0 | null | null |
| linux / environment_path_login / round 1 / code / shell ["Direct"] | request_to_first_byte_ns | 0 | null | null |
| linux / environment_path_login / round 1 / code / shell ["Direct"] | sandbox_setup_spawn_ns | 10 | 1.0197128701589984 | [0.9902307599860642,1.0530145182698238] |
| linux / environment_path_login / round 1 / code / shell ["Direct"] | spawn_return_ns | 10 | 0.9295241105617872 | [0.8299332272260781,1.0539269370279327] |
| linux / environment_path_login / round 1 / code / shell ["Direct"] | startup_to_ready_ns | 10 | 1.041485313044049 | [0.9728401091787714,1.1217736669642535] |
| linux / environment_path_login / round 1 / code / shell ["Direct"] | tool_call_ns | 10 | 1.013160601069357 | [0.9834494601855316,1.0438923073188495] |
| linux / environment_path_login / round 1 / code / shell ["Direct"] | unexplained_ns | 10 | 1.0000678784989427 | [0.9992580345441208,1.0008184993350675] |
| linux / environment_path_login / round 1 / code / shell ["Direct"] | wait_to_last_observed_eof_ns | 0 | null | null |
| linux / exec_missing_permission / round 1 / code / shell ["Direct","Direct"] | approval_ns | 10 | 1.038333110725257 | [0.7634422093039466,1.3804616805170822] |
| linux / exec_missing_permission / round 1 / code / shell ["Direct","Direct"] | artifact_write_ns | 10 | 0.9297854628906732 | [0.27889608797475157,2.793885470139312] |
| linux / exec_missing_permission / round 1 / code / shell ["Direct","Direct"] | codex_total_ns | 10 | 1.0000139695270611 | [0.9992999201598796,1.0008029534128726] |
| linux / exec_missing_permission / round 1 / code / shell ["Direct","Direct"] | external_response_ns | 0 | null | null |
| linux / exec_missing_permission / round 1 / code / shell ["Direct","Direct"] | fixture_runtime_ns | 0 | null | null |
| linux / exec_missing_permission / round 1 / code / shell ["Direct","Direct"] | launcher_child_spawn_ns | 0 | null | null |
| linux / exec_missing_permission / round 1 / code / shell ["Direct","Direct"] | launcher_prepare_ns | 0 | null | null |
| linux / exec_missing_permission / round 1 / code / shell ["Direct","Direct"] | policy_sandbox_spawn_ns | 10 | 1.0224680802312454 | [1.009769835752826,1.0347076373101862] |
| linux / exec_missing_permission / round 1 / code / shell ["Direct","Direct"] | rate_wait_ns | 0 | null | null |
| linux / exec_missing_permission / round 1 / code / shell ["Direct","Direct"] | ready_to_wait_observed_ns | 0 | null | null |
| linux / exec_missing_permission / round 1 / code / shell ["Direct","Direct"] | request_gate_queue_ns | 0 | null | null |
| linux / exec_missing_permission / round 1 / code / shell ["Direct","Direct"] | request_to_first_byte_ns | 0 | null | null |
| linux / exec_missing_permission / round 1 / code / shell ["Direct","Direct"] | sandbox_setup_spawn_ns | 10 | 1.022417957589432 | [1.0096653641397566,1.0346723915255434] |
| linux / exec_missing_permission / round 1 / code / shell ["Direct","Direct"] | spawn_return_ns | 10 | 1.0330927607095215 | [0.9582069210292404,1.1299739091105632] |
| linux / exec_missing_permission / round 1 / code / shell ["Direct","Direct"] | startup_to_ready_ns | 0 | null | null |
| linux / exec_missing_permission / round 1 / code / shell ["Direct","Direct"] | tool_call_ns | 10 | 1.0235562868480146 | [1.003417167434664,1.0431646155887553] |
| linux / exec_missing_permission / round 1 / code / shell ["Direct","Direct"] | unexplained_ns | 10 | 0.9997960938018836 | [0.9989960615024222,1.0006810251802118] |
| linux / exec_missing_permission / round 1 / code / shell ["Direct","Direct"] | wait_to_last_observed_eof_ns | 0 | null | null |
| linux / exit_code_vs_signal / round 1 / code / shell ["Direct","Direct"] | approval_ns | 10 | 1.0172327006780564 | [0.832445178652314,1.231131019036954] |
| linux / exit_code_vs_signal / round 1 / code / shell ["Direct","Direct"] | artifact_write_ns | 10 | 0.5920297928541417 | [0.3134821559973741,1.1041306237590194] |
| linux / exit_code_vs_signal / round 1 / code / shell ["Direct","Direct"] | codex_total_ns | 10 | 0.9288207121148258 | [0.7730086074672446,1.0006768181286212] |
| linux / exit_code_vs_signal / round 1 / code / shell ["Direct","Direct"] | external_response_ns | 0 | null | null |
| linux / exit_code_vs_signal / round 1 / code / shell ["Direct","Direct"] | fixture_runtime_ns | 10 | 0.973914687532234 | [0.8732569033468067,1.0655563097980734] |
| linux / exit_code_vs_signal / round 1 / code / shell ["Direct","Direct"] | launcher_child_spawn_ns | 0 | null | null |
| linux / exit_code_vs_signal / round 1 / code / shell ["Direct","Direct"] | launcher_prepare_ns | 0 | null | null |
| linux / exit_code_vs_signal / round 1 / code / shell ["Direct","Direct"] | policy_sandbox_spawn_ns | 10 | 1.0317285991592442 | [1.0186904111796973,1.0442046612635576] |
| linux / exit_code_vs_signal / round 1 / code / shell ["Direct","Direct"] | rate_wait_ns | 0 | null | null |
| linux / exit_code_vs_signal / round 1 / code / shell ["Direct","Direct"] | ready_to_wait_observed_ns | 10 | 1.054928904774242 | [0.9926011467703084,1.113156644318024] |
| linux / exit_code_vs_signal / round 1 / code / shell ["Direct","Direct"] | request_gate_queue_ns | 0 | null | null |
| linux / exit_code_vs_signal / round 1 / code / shell ["Direct","Direct"] | request_to_first_byte_ns | 0 | null | null |
| linux / exit_code_vs_signal / round 1 / code / shell ["Direct","Direct"] | sandbox_setup_spawn_ns | 10 | 1.0317896534937852 | [1.0185081611306386,1.0446109295400765] |
| linux / exit_code_vs_signal / round 1 / code / shell ["Direct","Direct"] | spawn_return_ns | 10 | 0.9696448686703236 | [0.9307137362470248,1.0115515107507198] |
| linux / exit_code_vs_signal / round 1 / code / shell ["Direct","Direct"] | startup_to_ready_ns | 10 | 1.055984309075526 | [1.027422063240138,1.0855775560697276] |
| linux / exit_code_vs_signal / round 1 / code / shell ["Direct","Direct"] | tool_call_ns | 10 | 1.0248888818177804 | [1.0039317988187144,1.0473294027333109] |
| linux / exit_code_vs_signal / round 1 / code / shell ["Direct","Direct"] | unexplained_ns | 10 | 0.927835532404722 | [0.7701912386645474,1.0004776744978934] |
| linux / exit_code_vs_signal / round 1 / code / shell ["Direct","Direct"] | wait_to_last_observed_eof_ns | 0 | null | null |
| linux / exit_nonzero / round 1 / code / shell ["Direct"] | approval_ns | 10 | 0.9517809923782858 | [0.6987885462555066,1.407908597358399] |
| linux / exit_nonzero / round 1 / code / shell ["Direct"] | artifact_write_ns | 10 | 1.3631922916900994 | [0.5003881490407208,2.8225025622086437] |
| linux / exit_nonzero / round 1 / code / shell ["Direct"] | codex_total_ns | 10 | 1.0004192395647036 | [0.9999412992933738,1.001015745243241] |
| linux / exit_nonzero / round 1 / code / shell ["Direct"] | external_response_ns | 0 | null | null |
| linux / exit_nonzero / round 1 / code / shell ["Direct"] | fixture_runtime_ns | 10 | 1.091091467708519 | [1.0032231217735157,1.2046112556502775] |
| linux / exit_nonzero / round 1 / code / shell ["Direct"] | launcher_child_spawn_ns | 0 | null | null |
| linux / exit_nonzero / round 1 / code / shell ["Direct"] | launcher_prepare_ns | 0 | null | null |
| linux / exit_nonzero / round 1 / code / shell ["Direct"] | policy_sandbox_spawn_ns | 10 | 1.0363982685697024 | [1.0090032989701,1.0610011170394962] |
| linux / exit_nonzero / round 1 / code / shell ["Direct"] | rate_wait_ns | 0 | null | null |
| linux / exit_nonzero / round 1 / code / shell ["Direct"] | ready_to_wait_observed_ns | 10 | 1.056834691825847 | [0.998576740638763,1.1155616715824015] |
| linux / exit_nonzero / round 1 / code / shell ["Direct"] | request_gate_queue_ns | 0 | null | null |
| linux / exit_nonzero / round 1 / code / shell ["Direct"] | request_to_first_byte_ns | 0 | null | null |
| linux / exit_nonzero / round 1 / code / shell ["Direct"] | sandbox_setup_spawn_ns | 10 | 1.0367374828499654 | [1.0096338854468212,1.061220007809587] |
| linux / exit_nonzero / round 1 / code / shell ["Direct"] | spawn_return_ns | 10 | 0.9706390186451032 | [0.9008303370377936,1.0463117133169098] |
| linux / exit_nonzero / round 1 / code / shell ["Direct"] | startup_to_ready_ns | 10 | 1.070815322797341 | [1.0206713385498702,1.1176850193512955] |
| linux / exit_nonzero / round 1 / code / shell ["Direct"] | tool_call_ns | 10 | 1.004308195453087 | [0.966807490577296,1.0441758381432464] |
| linux / exit_nonzero / round 1 / code / shell ["Direct"] | unexplained_ns | 10 | 1.0003981087447331 | [1.0000222149164757,1.0008733785284978] |
| linux / exit_nonzero / round 1 / code / shell ["Direct"] | wait_to_last_observed_eof_ns | 0 | null | null |
| linux / output_interleaved / round 1 / code / shell ["Direct"] | approval_ns | 10 | 0.8911211924821776 | [0.7079393006377832,1.1477394878280114] |
| linux / output_interleaved / round 1 / code / shell ["Direct"] | artifact_write_ns | 10 | 1.047724151286659 | [0.5891430727739652,1.8574313759201035] |
| linux / output_interleaved / round 1 / code / shell ["Direct"] | codex_total_ns | 10 | 1.0002803204323083 | [0.9999440538239792,1.000608972240548] |
| linux / output_interleaved / round 1 / code / shell ["Direct"] | external_response_ns | 0 | null | null |
| linux / output_interleaved / round 1 / code / shell ["Direct"] | fixture_runtime_ns | 10 | 1.0098169690019123 | [0.9981746043845996,1.0214325972510183] |
| linux / output_interleaved / round 1 / code / shell ["Direct"] | launcher_child_spawn_ns | 0 | null | null |
| linux / output_interleaved / round 1 / code / shell ["Direct"] | launcher_prepare_ns | 0 | null | null |
| linux / output_interleaved / round 1 / code / shell ["Direct"] | policy_sandbox_spawn_ns | 10 | 1.027486477232365 | [1.014164484611223,1.040322331490735] |
| linux / output_interleaved / round 1 / code / shell ["Direct"] | rate_wait_ns | 0 | null | null |
| linux / output_interleaved / round 1 / code / shell ["Direct"] | ready_to_wait_observed_ns | 10 | 1.0173605988634984 | [1.004326120122358,1.030781943493277] |
| linux / output_interleaved / round 1 / code / shell ["Direct"] | request_gate_queue_ns | 0 | null | null |
| linux / output_interleaved / round 1 / code / shell ["Direct"] | request_to_first_byte_ns | 0 | null | null |
| linux / output_interleaved / round 1 / code / shell ["Direct"] | sandbox_setup_spawn_ns | 10 | 1.027594039228552 | [1.0142631487794762,1.0403849674246328] |
| linux / output_interleaved / round 1 / code / shell ["Direct"] | spawn_return_ns | 10 | 0.955832907003142 | [0.8634656894772599,1.05144693788104] |
| linux / output_interleaved / round 1 / code / shell ["Direct"] | startup_to_ready_ns | 10 | 1.0951287905155085 | [1.0403137435880816,1.149241103351074] |
| linux / output_interleaved / round 1 / code / shell ["Direct"] | tool_call_ns | 10 | 1.0179916648746392 | [1.0004696100751445,1.0347122937400537] |
| linux / output_interleaved / round 1 / code / shell ["Direct"] | unexplained_ns | 10 | 1.0000640052570755 | [0.999656495803746,1.000459584230265] |
| linux / output_interleaved / round 1 / code / shell ["Direct"] | wait_to_last_observed_eof_ns | 0 | null | null |
| linux / output_split_large / round 1 / code / shell ["Direct"] | approval_ns | 10 | 1.0751937507688525 | [0.7741662024366023,1.424688680565272] |
| linux / output_split_large / round 1 / code / shell ["Direct"] | artifact_write_ns | 10 | 1.0219489682082332 | [0.7376295804241383,1.3444608610073068] |
| linux / output_split_large / round 1 / code / shell ["Direct"] | codex_total_ns | 10 | 1.0171180966627793 | [0.9998102722684687,1.053115601980663] |
| linux / output_split_large / round 1 / code / shell ["Direct"] | external_response_ns | 0 | null | null |
| linux / output_split_large / round 1 / code / shell ["Direct"] | fixture_runtime_ns | 10 | 0.9364403364799496 | [0.8502084991199185,1.0154021918072875] |
| linux / output_split_large / round 1 / code / shell ["Direct"] | launcher_child_spawn_ns | 0 | null | null |
| linux / output_split_large / round 1 / code / shell ["Direct"] | launcher_prepare_ns | 0 | null | null |
| linux / output_split_large / round 1 / code / shell ["Direct"] | policy_sandbox_spawn_ns | 10 | 1.0105002753642744 | [0.9909373881342683,1.0282950506545574] |
| linux / output_split_large / round 1 / code / shell ["Direct"] | rate_wait_ns | 0 | null | null |
| linux / output_split_large / round 1 / code / shell ["Direct"] | ready_to_wait_observed_ns | 10 | 0.9819292301496994 | [0.9182928175134578,1.0390633391548134] |
| linux / output_split_large / round 1 / code / shell ["Direct"] | request_gate_queue_ns | 0 | null | null |
| linux / output_split_large / round 1 / code / shell ["Direct"] | request_to_first_byte_ns | 0 | null | null |
| linux / output_split_large / round 1 / code / shell ["Direct"] | sandbox_setup_spawn_ns | 10 | 1.010588383952392 | [0.9909023129053168,1.0284894606005228] |
| linux / output_split_large / round 1 / code / shell ["Direct"] | spawn_return_ns | 10 | 0.9487749465795772 | [0.8614311527549527,1.0506312077181263] |
| linux / output_split_large / round 1 / code / shell ["Direct"] | startup_to_ready_ns | 10 | 1.0263608053744204 | [0.991454177771236,1.0611053542539115] |
| linux / output_split_large / round 1 / code / shell ["Direct"] | tool_call_ns | 10 | 1.01491462361881 | [0.9936693452179608,1.0348488002659797] |
| linux / output_split_large / round 1 / code / shell ["Direct"] | unexplained_ns | 10 | 1.017130303366925 | [0.999683688307456,1.0534453106566617] |
| linux / output_split_large / round 1 / code / shell ["Direct"] | wait_to_last_observed_eof_ns | 0 | null | null |
| linux / output_truncation_tail / round 1 / code / shell ["Direct"] | approval_ns | 10 | 0.935023975798925 | [0.7085231154634479,1.2527275337352857] |
| linux / output_truncation_tail / round 1 / code / shell ["Direct"] | artifact_write_ns | 10 | 0.5680387213740897 | [0.3276151706308894,0.9113810737837744] |
| linux / output_truncation_tail / round 1 / code / shell ["Direct"] | codex_total_ns | 10 | 1.000532989884147 | [1.000051053265561,1.0009590157684454] |
| linux / output_truncation_tail / round 1 / code / shell ["Direct"] | external_response_ns | 0 | null | null |
| linux / output_truncation_tail / round 1 / code / shell ["Direct"] | fixture_runtime_ns | 10 | 1.0448168683263137 | [0.9454750256894396,1.148752120574212] |
| linux / output_truncation_tail / round 1 / code / shell ["Direct"] | launcher_child_spawn_ns | 0 | null | null |
| linux / output_truncation_tail / round 1 / code / shell ["Direct"] | launcher_prepare_ns | 0 | null | null |
| linux / output_truncation_tail / round 1 / code / shell ["Direct"] | policy_sandbox_spawn_ns | 10 | 1.0273982647036926 | [1.0000930363061304,1.049559519965903] |
| linux / output_truncation_tail / round 1 / code / shell ["Direct"] | rate_wait_ns | 0 | null | null |
| linux / output_truncation_tail / round 1 / code / shell ["Direct"] | ready_to_wait_observed_ns | 10 | 1.0449493060394175 | [0.9817574587995574,1.1087641927999283] |
| linux / output_truncation_tail / round 1 / code / shell ["Direct"] | request_gate_queue_ns | 0 | null | null |
| linux / output_truncation_tail / round 1 / code / shell ["Direct"] | request_to_first_byte_ns | 0 | null | null |
| linux / output_truncation_tail / round 1 / code / shell ["Direct"] | sandbox_setup_spawn_ns | 10 | 1.0278241154377463 | [1.0003910028426115,1.049732268403317] |
| linux / output_truncation_tail / round 1 / code / shell ["Direct"] | spawn_return_ns | 10 | 0.9444261728599675 | [0.861561939542603,1.036598653264223] |
| linux / output_truncation_tail / round 1 / code / shell ["Direct"] | startup_to_ready_ns | 10 | 1.0581199203240366 | [1.0068457374810489,1.0981127362136327] |
| linux / output_truncation_tail / round 1 / code / shell ["Direct"] | tool_call_ns | 10 | 1.014727705010957 | [0.9876997440370942,1.0467359086011392] |
| linux / output_truncation_tail / round 1 / code / shell ["Direct"] | unexplained_ns | 10 | 1.0004448635781351 | [0.9999265925872084,1.0008836607908185] |
| linux / output_truncation_tail / round 1 / code / shell ["Direct"] | wait_to_last_observed_eof_ns | 0 | null | null |
| linux / session_cancel_resources / round 1 / code / shell ["Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct"] | approval_ns | 10 | 1.065799231125979 | [0.9294250744756132,1.2424168370654831] |
| linux / session_cancel_resources / round 1 / code / shell ["Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct"] | artifact_write_ns | 10 | 1.2411372348004623 | [0.5415514703474319,2.3894385488844385] |
| linux / session_cancel_resources / round 1 / code / shell ["Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct"] | codex_total_ns | 10 | 1.0243873432313877 | [0.9993111669960666,1.0806092297872985] |
| linux / session_cancel_resources / round 1 / code / shell ["Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct"] | external_response_ns | 0 | null | null |
| linux / session_cancel_resources / round 1 / code / shell ["Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct"] | fixture_runtime_ns | 10 | 0.9969149510532234 | [0.6626178842371546,1.3467958225444296] |
| linux / session_cancel_resources / round 1 / code / shell ["Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct"] | launcher_child_spawn_ns | 0 | null | null |
| linux / session_cancel_resources / round 1 / code / shell ["Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct"] | launcher_prepare_ns | 0 | null | null |
| linux / session_cancel_resources / round 1 / code / shell ["Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct"] | policy_sandbox_spawn_ns | 10 | 1.0055813836681613 | [0.980450303856048,1.033380449859412] |
| linux / session_cancel_resources / round 1 / code / shell ["Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct"] | rate_wait_ns | 0 | null | null |
| linux / session_cancel_resources / round 1 / code / shell ["Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct"] | ready_to_wait_observed_ns | 10 | 1.0000617076342744 | [0.995409978671362,1.0052632324201467] |
| linux / session_cancel_resources / round 1 / code / shell ["Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct"] | request_gate_queue_ns | 0 | null | null |
| linux / session_cancel_resources / round 1 / code / shell ["Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct"] | request_to_first_byte_ns | 0 | null | null |
| linux / session_cancel_resources / round 1 / code / shell ["Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct"] | sandbox_setup_spawn_ns | 10 | 1.005566279022286 | [0.9804278353276292,1.0333431367469146] |
| linux / session_cancel_resources / round 1 / code / shell ["Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct"] | spawn_return_ns | 10 | 0.9824890880996554 | [0.9463176234830752,1.016798169452739] |
| linux / session_cancel_resources / round 1 / code / shell ["Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct"] | startup_to_ready_ns | 10 | 1.024065084516585 | [1.01036976683922,1.0376647049889576] |
| linux / session_cancel_resources / round 1 / code / shell ["Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct"] | tool_call_ns | 10 | 1.0010847940848735 | [0.996620335137775,1.006052910478788] |
| linux / session_cancel_resources / round 1 / code / shell ["Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct"] | unexplained_ns | 10 | 1.0426552939844835 | [1.0002293891466196,1.157009016183754] |
| linux / session_cancel_resources / round 1 / code / shell ["Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct"] | wait_to_last_observed_eof_ns | 10 | 0.7774576192188354 | [0.4694292286593631,1.2213319889620038] |
| linux / session_failure_recovery / round 1 / code / shell ["Direct","Direct"] | approval_ns | 10 | 1.7407708097178396 | [1.5085797395388556,1.9883841807909604] |
| linux / session_failure_recovery / round 1 / code / shell ["Direct","Direct"] | artifact_write_ns | 10 | 0.6918332288498464 | [0.24707244813938295,1.948794838408493] |
| linux / session_failure_recovery / round 1 / code / shell ["Direct","Direct"] | codex_total_ns | 10 | 1.0119163815306345 | [0.999981774958358,1.0449393938957965] |
| linux / session_failure_recovery / round 1 / code / shell ["Direct","Direct"] | external_response_ns | 0 | null | null |
| linux / session_failure_recovery / round 1 / code / shell ["Direct","Direct"] | fixture_runtime_ns | 10 | 0.9918570133915082 | [0.9576468606312036,1.0402670031959131] |
| linux / session_failure_recovery / round 1 / code / shell ["Direct","Direct"] | launcher_child_spawn_ns | 0 | null | null |
| linux / session_failure_recovery / round 1 / code / shell ["Direct","Direct"] | launcher_prepare_ns | 0 | null | null |
| linux / session_failure_recovery / round 1 / code / shell ["Direct","Direct"] | policy_sandbox_spawn_ns | 10 | 1.0293877801787163 | [1.0116442539621229,1.0472520750903938] |
| linux / session_failure_recovery / round 1 / code / shell ["Direct","Direct"] | rate_wait_ns | 0 | null | null |
| linux / session_failure_recovery / round 1 / code / shell ["Direct","Direct"] | ready_to_wait_observed_ns | 10 | 1.045403934292463 | [1.0188208827986718,1.0733509955152385] |
| linux / session_failure_recovery / round 1 / code / shell ["Direct","Direct"] | request_gate_queue_ns | 0 | null | null |
| linux / session_failure_recovery / round 1 / code / shell ["Direct","Direct"] | request_to_first_byte_ns | 0 | null | null |
| linux / session_failure_recovery / round 1 / code / shell ["Direct","Direct"] | sandbox_setup_spawn_ns | 10 | 1.0290761206808512 | [1.0113766373412831,1.04688772864461] |
| linux / session_failure_recovery / round 1 / code / shell ["Direct","Direct"] | spawn_return_ns | 10 | 1.002560615902574 | [0.9539340323068796,1.042167433313599] |
| linux / session_failure_recovery / round 1 / code / shell ["Direct","Direct"] | startup_to_ready_ns | 10 | 1.0541134259099854 | [1.0182314342754706,1.091604034189622] |
| linux / session_failure_recovery / round 1 / code / shell ["Direct","Direct"] | tool_call_ns | 10 | 1.041126870006432 | [1.022099863687094,1.0606691077592385] |
| linux / session_failure_recovery / round 1 / code / shell ["Direct","Direct"] | unexplained_ns | 10 | 1.0115936263447156 | [0.9996419131088612,1.0449212589206525] |
| linux / session_failure_recovery / round 1 / code / shell ["Direct","Direct"] | wait_to_last_observed_eof_ns | 0 | null | null |
| linux / session_poll / round 1 / code / shell ["Direct"] | approval_ns | 10 | 1.1711681041004691 | [0.9271660108050424,1.443331664162911] |
| linux / session_poll / round 1 / code / shell ["Direct"] | artifact_write_ns | 10 | 0.6629830917226909 | [0.30049047699061165,1.0769712501351505] |
| linux / session_poll / round 1 / code / shell ["Direct"] | codex_total_ns | 10 | 0.9196365243157324 | [0.7588606300949197,1.0012233445697063] |
| linux / session_poll / round 1 / code / shell ["Direct"] | external_response_ns | 0 | null | null |
| linux / session_poll / round 1 / code / shell ["Direct"] | fixture_runtime_ns | 10 | 0.9998856169012084 | [0.9997532938277226,1.0000079343056003] |
| linux / session_poll / round 1 / code / shell ["Direct"] | launcher_child_spawn_ns | 0 | null | null |
| linux / session_poll / round 1 / code / shell ["Direct"] | launcher_prepare_ns | 0 | null | null |
| linux / session_poll / round 1 / code / shell ["Direct"] | policy_sandbox_spawn_ns | 10 | 1.000467081381654 | [0.998215405869223,1.0023806011785297] |
| linux / session_poll / round 1 / code / shell ["Direct"] | rate_wait_ns | 0 | null | null |
| linux / session_poll / round 1 / code / shell ["Direct"] | ready_to_wait_observed_ns | 10 | 0.9981382871790158 | [0.9859157329611482,1.0091693545619214] |
| linux / session_poll / round 1 / code / shell ["Direct"] | request_gate_queue_ns | 0 | null | null |
| linux / session_poll / round 1 / code / shell ["Direct"] | request_to_first_byte_ns | 0 | null | null |
| linux / session_poll / round 1 / code / shell ["Direct"] | sandbox_setup_spawn_ns | 10 | 1.0005025726035357 | [0.9982655580793354,1.002408559059519] |
| linux / session_poll / round 1 / code / shell ["Direct"] | spawn_return_ns | 10 | 0.976038532880105 | [0.8634775259599773,1.088172284448076] |
| linux / session_poll / round 1 / code / shell ["Direct"] | startup_to_ready_ns | 10 | 1.037120368902162 | [0.984757359077532,1.0919477214502502] |
| linux / session_poll / round 1 / code / shell ["Direct"] | tool_call_ns | 10 | 0.9982195967613378 | [0.9862223346946832,1.0094761352827146] |
| linux / session_poll / round 1 / code / shell ["Direct"] | unexplained_ns | 10 | 0.902268136829904 | [0.7070043573764746,1.0000681231956363] |
| linux / session_poll / round 1 / code / shell ["Direct"] | wait_to_last_observed_eof_ns | 3 | 1.4641977797295935 | [0.5184318936877076,6.119291569086651] |
| linux / session_sequence / round 1 / code / shell ["Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct"] | approval_ns | 10 | 1.048442993000181 | [0.8871537876415052,1.2560022977219822] |
| linux / session_sequence / round 1 / code / shell ["Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct"] | artifact_write_ns | 10 | 0.7260248527675479 | [0.19331203114058376,2.335859298269813] |
| linux / session_sequence / round 1 / code / shell ["Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct"] | codex_total_ns | 10 | 1.0017257993118414 | [1.000008502159513,1.0039948122797469] |
| linux / session_sequence / round 1 / code / shell ["Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct"] | external_response_ns | 0 | null | null |
| linux / session_sequence / round 1 / code / shell ["Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct"] | fixture_runtime_ns | 10 | 1.4646673186906085 | [0.8350855413430198,2.676189750317802] |
| linux / session_sequence / round 1 / code / shell ["Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct"] | launcher_child_spawn_ns | 0 | null | null |
| linux / session_sequence / round 1 / code / shell ["Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct"] | launcher_prepare_ns | 0 | null | null |
| linux / session_sequence / round 1 / code / shell ["Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct"] | policy_sandbox_spawn_ns | 10 | 1.034774746540765 | [1.003528639620615,1.0811000514870717] |
| linux / session_sequence / round 1 / code / shell ["Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct"] | rate_wait_ns | 0 | null | null |
| linux / session_sequence / round 1 / code / shell ["Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct"] | ready_to_wait_observed_ns | 10 | 1.2300112297518 | [0.9477281310038882,1.738662724869254] |
| linux / session_sequence / round 1 / code / shell ["Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct"] | request_gate_queue_ns | 0 | null | null |
| linux / session_sequence / round 1 / code / shell ["Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct"] | request_to_first_byte_ns | 0 | null | null |
| linux / session_sequence / round 1 / code / shell ["Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct"] | sandbox_setup_spawn_ns | 10 | 1.0347900448217666 | [1.003739640162458,1.081265778511625] |
| linux / session_sequence / round 1 / code / shell ["Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct"] | spawn_return_ns | 10 | 0.9798907463025368 | [0.937983170176098,1.0314247670833108] |
| linux / session_sequence / round 1 / code / shell ["Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct"] | startup_to_ready_ns | 10 | 1.0227732026106773 | [0.998191041503926,1.0478115663142846] |
| linux / session_sequence / round 1 / code / shell ["Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct"] | tool_call_ns | 10 | 1.033009563538838 | [1.0009769001951334,1.0790600630996043] |
| linux / session_sequence / round 1 / code / shell ["Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct"] | unexplained_ns | 10 | 1.0002197685556038 | [0.9996390866682698,1.000729285474251] |
| linux / session_sequence / round 1 / code / shell ["Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct","Direct"] | wait_to_last_observed_eof_ns | 0 | null | null |
| linux / stdin_backpressure / round 1 / code / shell ["Direct"] | approval_ns | 10 | 1.013135017459784 | [0.7856310835598281,1.3022926604092937] |
| linux / stdin_backpressure / round 1 / code / shell ["Direct"] | artifact_write_ns | 10 | 1.127429019508622 | [0.6235417664202535,2.2195748026560156] |
| linux / stdin_backpressure / round 1 / code / shell ["Direct"] | codex_total_ns | 10 | 1.0002068924794705 | [0.9996325340189092,1.0008711250699076] |
| linux / stdin_backpressure / round 1 / code / shell ["Direct"] | external_response_ns | 0 | null | null |
| linux / stdin_backpressure / round 1 / code / shell ["Direct"] | fixture_runtime_ns | 10 | 0.924845657346777 | [0.8273715798128717,1.0472706723025258] |
| linux / stdin_backpressure / round 1 / code / shell ["Direct"] | launcher_child_spawn_ns | 0 | null | null |
| linux / stdin_backpressure / round 1 / code / shell ["Direct"] | launcher_prepare_ns | 0 | null | null |
| linux / stdin_backpressure / round 1 / code / shell ["Direct"] | policy_sandbox_spawn_ns | 10 | 1.0429262740759566 | [1.0153452835741847,1.0706352894590772] |
| linux / stdin_backpressure / round 1 / code / shell ["Direct"] | rate_wait_ns | 0 | null | null |
| linux / stdin_backpressure / round 1 / code / shell ["Direct"] | ready_to_wait_observed_ns | 10 | 1.017330257289737 | [0.9328525072836044,1.1203933225931029] |
| linux / stdin_backpressure / round 1 / code / shell ["Direct"] | request_gate_queue_ns | 0 | null | null |
| linux / stdin_backpressure / round 1 / code / shell ["Direct"] | request_to_first_byte_ns | 0 | null | null |
| linux / stdin_backpressure / round 1 / code / shell ["Direct"] | sandbox_setup_spawn_ns | 10 | 1.0430921414090792 | [1.0155533248033417,1.0707452656573035] |
| linux / stdin_backpressure / round 1 / code / shell ["Direct"] | spawn_return_ns | 10 | 1.0295226726173523 | [0.9551415640687504,1.1126915279723937] |
| linux / stdin_backpressure / round 1 / code / shell ["Direct"] | startup_to_ready_ns | 10 | 1.0938707638654737 | [1.0458904008644925,1.140622698308073] |
| linux / stdin_backpressure / round 1 / code / shell ["Direct"] | tool_call_ns | 10 | 1.0294323323607424 | [0.9997355245247364,1.0575359743779127] |
| linux / stdin_backpressure / round 1 / code / shell ["Direct"] | unexplained_ns | 10 | 1.0000398388982652 | [0.9995180542884504,1.000658872239845] |
| linux / stdin_backpressure / round 1 / code / shell ["Direct"] | wait_to_last_observed_eof_ns | 0 | null | null |
| linux / stdin_eof / round 1 / code / shell ["Direct"] | approval_ns | 10 | 0.9865138917055364 | [0.7601033400996494,1.273990494503501] |
| linux / stdin_eof / round 1 / code / shell ["Direct"] | artifact_write_ns | 10 | 2.026321497855066 | [0.6322828358334962,6.0695265205659545] |
| linux / stdin_eof / round 1 / code / shell ["Direct"] | codex_total_ns | 10 | 0.9995314131995656 | [0.9977958452387944,1.000576729133342] |
| linux / stdin_eof / round 1 / code / shell ["Direct"] | external_response_ns | 0 | null | null |
| linux / stdin_eof / round 1 / code / shell ["Direct"] | fixture_runtime_ns | 10 | 0.952810912874194 | [0.882058276231162,1.0349865793382167] |
| linux / stdin_eof / round 1 / code / shell ["Direct"] | launcher_child_spawn_ns | 0 | null | null |
| linux / stdin_eof / round 1 / code / shell ["Direct"] | launcher_prepare_ns | 0 | null | null |
| linux / stdin_eof / round 1 / code / shell ["Direct"] | policy_sandbox_spawn_ns | 10 | 1.0213523506064353 | [0.9955034725929348,1.0504545412634196] |
| linux / stdin_eof / round 1 / code / shell ["Direct"] | rate_wait_ns | 0 | null | null |
| linux / stdin_eof / round 1 / code / shell ["Direct"] | ready_to_wait_observed_ns | 10 | 1.0087571194517109 | [0.9475588054702084,1.0816030516912187] |
| linux / stdin_eof / round 1 / code / shell ["Direct"] | request_gate_queue_ns | 0 | null | null |
| linux / stdin_eof / round 1 / code / shell ["Direct"] | request_to_first_byte_ns | 0 | null | null |
| linux / stdin_eof / round 1 / code / shell ["Direct"] | sandbox_setup_spawn_ns | 10 | 1.0213667155641857 | [0.9952800240572788,1.0504537465179449] |
| linux / stdin_eof / round 1 / code / shell ["Direct"] | spawn_return_ns | 10 | 1.0078327461027192 | [0.9231363721863798,1.0874051908029496] |
| linux / stdin_eof / round 1 / code / shell ["Direct"] | startup_to_ready_ns | 10 | 1.044669192058391 | [0.9975028323647596,1.0969861057276018] |
| linux / stdin_eof / round 1 / code / shell ["Direct"] | tool_call_ns | 10 | 1.0234941494155116 | [0.9831607545219736,1.068445662376487] |
| linux / stdin_eof / round 1 / code / shell ["Direct"] | unexplained_ns | 10 | 0.9993998356006724 | [0.9976953661413708,1.0004692623859155] |
| linux / stdin_eof / round 1 / code / shell ["Direct"] | wait_to_last_observed_eof_ns | 0 | null | null |
| linux / stdin_pty_utf8 / round 1 / code / shell ["Direct"] | approval_ns | 10 | 1.0461258662318522 | [0.9202550415183868,1.2352390552995391] |
| linux / stdin_pty_utf8 / round 1 / code / shell ["Direct"] | artifact_write_ns | 10 | 0.8093082246765666 | [0.31632680947429515,2.491134985879149] |
| linux / stdin_pty_utf8 / round 1 / code / shell ["Direct"] | codex_total_ns | 10 | 1.000603417581255 | [0.9996127285341956,1.0015485890442284] |
| linux / stdin_pty_utf8 / round 1 / code / shell ["Direct"] | external_response_ns | 0 | null | null |
| linux / stdin_pty_utf8 / round 1 / code / shell ["Direct"] | fixture_runtime_ns | 10 | 0.999920749459061 | [0.999356525903066,1.0004538255183175] |
| linux / stdin_pty_utf8 / round 1 / code / shell ["Direct"] | launcher_child_spawn_ns | 0 | null | null |
| linux / stdin_pty_utf8 / round 1 / code / shell ["Direct"] | launcher_prepare_ns | 0 | null | null |
| linux / stdin_pty_utf8 / round 1 / code / shell ["Direct"] | policy_sandbox_spawn_ns | 10 | 1.0008431692966355 | [0.9983229680454532,1.0033955265180168] |
| linux / stdin_pty_utf8 / round 1 / code / shell ["Direct"] | rate_wait_ns | 0 | null | null |
| linux / stdin_pty_utf8 / round 1 / code / shell ["Direct"] | ready_to_wait_observed_ns | 10 | 0.9968187231305904 | [0.9904686597549598,1.0019913329068948] |
| linux / stdin_pty_utf8 / round 1 / code / shell ["Direct"] | request_gate_queue_ns | 0 | null | null |
| linux / stdin_pty_utf8 / round 1 / code / shell ["Direct"] | request_to_first_byte_ns | 0 | null | null |
| linux / stdin_pty_utf8 / round 1 / code / shell ["Direct"] | sandbox_setup_spawn_ns | 10 | 1.000752399608548 | [0.9982027783702928,1.003287110731072] |
| linux / stdin_pty_utf8 / round 1 / code / shell ["Direct"] | spawn_return_ns | 10 | 1.0201800034398572 | [0.949374241919846,1.111076578126561] |
| linux / stdin_pty_utf8 / round 1 / code / shell ["Direct"] | startup_to_ready_ns | 10 | 1.0027489937064673 | [0.947647339562136,1.0659919828206474] |
| linux / stdin_pty_utf8 / round 1 / code / shell ["Direct"] | tool_call_ns | 10 | 1.0001097826153884 | [0.9997333531827748,1.000475544427589] |
| linux / stdin_pty_utf8 / round 1 / code / shell ["Direct"] | unexplained_ns | 10 | 1.0007887581604678 | [0.9994924543451288,1.0021232371407929] |
| linux / stdin_pty_utf8 / round 1 / code / shell ["Direct"] | wait_to_last_observed_eof_ns | 1 | 0.5903275176002448 | [0.5903275176002448,0.5903275176002448] |
