useEffect(() => {
    const getVoteAccount = async () => {
      let account,
        accountBump = null
      ;[account, accountBump] = await web3.PublicKey.findProgramAddress(
        [Buffer.from("vote_account")],
        programID
      )
      setVoteAccount({ account, accountBump })
    }
    getVoteAccount()
  }, [])