from typing import Dict, List, Optional

import numpy as np
import triton_python_backend_utils as pb_utils
from transformers import AutoTokenizer, PreTrainedTokenizer


class TritonPythonModel:
    tokenizer: Optional[PreTrainedTokenizer]

    def initialize(self, args: Dict[str, str]) -> None:
        self.tokenizer = AutoTokenizer.from_pretrained('BAAI/bge-large-en-v1.5')

    def execute(self, requests) -> "List[List[pb_utils.Tensor]]":
        responses = []
        for request in requests:
            query: List[str] = [
                t.decode('utf-8')
                for t in pb_utils.get_input_tensor_by_name(request, 'text').as_numpy()[:, 0].tolist()
            ]

            tokens: Dict[str, np.ndarray] = self.tokenizer(
                text=query,
                padding=True,
                truncation=True,
                max_length=512,
                return_tensors='np',
            )

            inference_response = pb_utils.InferenceResponse(
                output_tensors=[
                    pb_utils.Tensor(input_name, tokens[input_name].astype(np.int64))
                    for input_name in ('input_ids', 'attention_mask', 'token_type_ids')
                ]
            )

            responses.append(inference_response)

        return responses

    def finalize(self, args):
        self.tokenizer = None
