export const idlFactory = ({ IDL }) => {
  const http_header = IDL.Tuple(IDL.Text, IDL.Text);
  const strategy = IDL.Variant({
    'Callback' : IDL.Record({
      'token' : IDL.Reserved,
      'callback' : IDL.Func([], [], []),
    }),
  });
  const http_response = IDL.Record({
    'body' : IDL.Vec(IDL.Nat8),
    'headers' : IDL.Vec(http_header),
    'upgrade' : IDL.Opt(IDL.Bool),
    'streaming_strategy' : IDL.Opt(strategy),
    'status_code' : IDL.Nat16,
  });
  return IDL.Service({
    'http_request' : IDL.Func(
        [
          IDL.Record({
            'url' : IDL.Text,
            'method' : IDL.Text,
            'body' : IDL.Opt(IDL.Vec(IDL.Nat8)),
            'headers' : IDL.Vec(http_header),
          }),
        ],
        [http_response],
        ['query'],
      ),
    'sign_digest_ic' : IDL.Func([IDL.Text], [IDL.Text], []),
    'sign_digest_mpc' : IDL.Func([IDL.Text, IDL.Text], [IDL.Text], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };