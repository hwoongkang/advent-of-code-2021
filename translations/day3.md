# 세번째 날: 이진 진단

잠수함에서 이상하게 삐걱거리는 소리가 나고 있어, 진단 리포트를 발행하도록 하였습니다.

이번 퍼즐의 입력값인 진단 리포트에는 이진수가 나열되어 있으며, 이를 해석하면 잠수함의 상태에 대한 쓸만한 정보들이 들어있습니다. 체크해야 할 첫 번째 상태값은 `에너지 사용량 (power consumption)`입니다.  

진단 리포트에 들어있는 이진수를 이용하면 `감마 비율(gamma rate)`과 `입실론 비율(epsilon rate)`을 구할 수 있습니다. 그 후엔 두 값으르 곱해 에너지 사용량을 구할 수 있습니다.

감마 비율의 각 비트는 진단 리포트에서 제공된 모든 비트에 대해, 각 자리에서 가장 많이 나타난 비트를 취하면 얻을 수 있습니다. 다음의 예시 입력에 대해:

 ```
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
```

각 숫자의 첫번째 비트만 살펴보면, `0`이 다섯 번, `1`이 일곱 번 나타났음을 알 수 있습니다. 따라서 1번이 더 많이 나타났으므로, 감마 비율의 첫번째 비트는 `1`이 됩니다.  

두번째 비트에서 더 많이 나타난 비트는 `0`이므로, 감마 비율의 두번째 비트는 `0`입니다.  

세번째, 네번째, 다섯번째 비트에서 더 많이 나타난 비트는 각각 `1`, `1`, `0` 이므로 감마 비율의 마지막 세 비트는 `110`이 됩니다.

따라서 감마비율은 이진수로 `10110`이라는 것을 구했고, 이는 십진수로 `22`입니다.  

입실론 비율은 반대로 덜 쓰인 비트는 취하면 얻을 수 있습니다. 즉 위의 예시에서 입실론 비율은 `01001`이고, 십진수로는 `9`입니다. 감마비율 `22`와 입실론 비율 `9`를 곱하면 이 경우 에너지 사용량은 `198`입니다.  

진단리포트의 이진수들을 이용하여 감마 비율과 입실론 비율을 구하고,  두 값을 곱하세요. 잠수함의 에너지 사용량을 십진수로 출력하세요.

— 

다음으로는 `생명 유지 등급`을 검증해야 합니다. 생명 유지 등급은 `산소 발생기 등급`과 `이산화탄소 수집기 등급`을 곱해 구할 수 있습니다.

산소 발생기 등급과 이산화탄소 수집기 등급은 모두 진단 리포트에서 구해낼 수 있는 값입니다. (조금 어렵긴 합니다.) 두 값은 모두 필요한 값 이외의 다른 값들을 지워나가는 과정을 통해 구할 수 있습니다. 각 등급을 구하기 전에 우선은 진단 리포트 안의 모든 이진수에 대해 **첫 번째 비트**만 고려하며 시작하면 됩니다.

- 구하고자 하는 등급의 `비트 조건`에 맞지 않는 수는 버립니다.
- 이진수가 딱 하나 남았다면 그게 구하고자 하는 등급입니다.
- 아니라면 오른쪽의 다음 비트에 대해 이 과정을 반복합니다.

**비트 조건**은 구하고자 하는 등급에 따라 달라집니다.

- 산소 발생기 등급을 구하기 위해서는, 지금 고려하고 있는 비트 위치에서 **더 많이 나타난 값**을 결정한 다음, 그 위치에서 그 비트를 가지고 있는 숫자만 남깁니다. `0`과 `1`이  같은 회수만큼 나타났다면, `1`을 가지고 있는 숫자를 남깁니다.
- 이산화탄소 수집기 등급의 경우에는 **더 조금 나타난 값***을 결정한 다음, 그 위치에 그 비트를 가지고 있는 숫자만을 남깁니다. `1`과 `0`이 같은 회수만큼 나타났다면 `0`을 가지고 있는 숫자를 남깁니다.

예를 들어 위에서 고려했던 예시 리포트에 대해 산소 발생기 등급을 구하는 과정은 다음과 같습니다.

- 12개 숫자를 모두 고려합니다. 첫번째 자리에서 `1`이 (7번) `0`보다 (5번) 더 많이 나타났으므로, `1`을 첫번째 비트로 가지는 숫자만 남깁니다: `11110`, `10110`, `10111`, `10101`, `11100`, `10000`, `11001`
- 남아있는 7개 숫자에 대해, 두번째 비트에서 `0`이 (4번) `1`보다 (3번) 더 자주 나타났으므로, 두번째 비트가 `0`인 숫자만 남깁니다: `10110`, `10111`, `10101`, `10000`
- 세번째 비트에 대해 네 개 중 세 개의 숫자가 1을 가지고 있으므로 그 세 숫자만 남깁니다: `10110`, `10111`, `10101`
- 네번째 비트에서는 세 개 중 두 개의 숫자가 1을 가지고 있으므로 그 두 숫자만 남깁니다: `10110`, `10111`
- 다섯번째 비트에서 `0`과 `1`이 같은 숫자만큼 나타납니다. 지금은 **산소 발생기 등급**을 구하고 있으므로 `1`을 가진 숫자를 남깁니다: `10111`
- 하나의 숫자만 남았으므로 멈춥니다. 산소 발생기 등급은 `10111`, 십진수로 `23`입니다.

이산화탄소 수집기 등급을 구하는 과정은 다음과 같습니다/

- 12개의 숫자를 다시 한 번 모두 고려합니다. 첫번째 비트에서 `0`이 더 적게 나타나므로, `0`을 가지는 숫자만 남깁니다: `00100`, `01111`, `00111`, `00010`, `01010`
- 두번째 비트에서 `1`이 (2번) `0`보다 (3번) 더 적게 나타납니다. 따라서 `1`을 가진 숫자만 남깁니다: `01111`, `01010`
- 세번째 비트에서 `1`과 `0`이 같은 회수 나타납니다. 지금은 **이산화탄소 수집기 등급**을 구하고 있으므로, `0`을 가진 수만 남깁니다:  `01010`
- 숫자가 하나만 남았으므로 멈춥니다. **이산화탄소 수집기 등급**은 01010, 또는 십진수로 `10`입니다.

두 등급을 곱하면 생명 유지 등급은 `230`입니다.


