package com.mahala.blockchain;

import com.mahala.wallet.PersonalWallet;
import com.mahala.wallet.WalletManager;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.math.BigDecimal;
import java.time.LocalDate;
import java.util.List;
import java.util.concurrent.Executors;
import java.util.concurrent.ScheduledExecutorService;
import java.util.concurrent.TimeUnit;

public class TokenDistribution {
    private static final Logger logger = LoggerFactory.getLogger(TokenDistribution.class);

    private static final BigDecimal DAILY_TOKEN_AMOUNT = new BigDecimal("10.0");

    private final WalletManager walletManager;
    private final ScheduledExecutorService scheduler;
    private LocalDate lastDistributionDate;

    public TokenDistribution(WalletManager walletManager) {
        this.walletManager = walletManager;
        this.scheduler = Executors.newScheduledThreadPool(1);
        this.lastDistributionDate = LocalDate.now().minusDays(1);
    }

    public void startDailyDistribution() {
        scheduler.scheduleAtFixedRate(this::distributeTokens, 0, 24, TimeUnit.HOURS);
        logger.info("Daily token distribution started");
    }

    public void stopDailyDistribution() {
        scheduler.shutdown();
        logger.info("Daily token distribution stopped");
    }

    public void distributeTokens() {
        LocalDate today = LocalDate.now();

        if (today.equals(lastDistributionDate)) {
            logger.debug("Tokens already distributed today");
            return;
        }

        List<PersonalWallet> personalWallets = walletManager.getAllPersonalWallets();

        if (personalWallets.isEmpty()) {
            logger.info("No personal wallets found for token distribution");
            return;
        }

        int distributedCount = 0;
        for (PersonalWallet wallet : personalWallets) {
            if (wallet.isActive() && wallet.canReceiveTokens()) {
                wallet.addTokens(DAILY_TOKEN_AMOUNT);
                distributedCount++;
            }
        }

        lastDistributionDate = today;

        logger.info("Distributed {} Mahala tokens to {} personal wallets on {}",
                   DAILY_TOKEN_AMOUNT, distributedCount, today);
    }

    public BigDecimal getDailyTokenAmount() {
        return DAILY_TOKEN_AMOUNT;
    }

    public LocalDate getLastDistributionDate() {
        return lastDistributionDate;
    }

    public boolean hasDistributedToday() {
        return LocalDate.now().equals(lastDistributionDate);
    }
}