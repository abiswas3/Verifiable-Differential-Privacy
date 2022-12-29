import logging
import pandas as pd
import numpy as np
from collections import Counter
logger = logging.getLogger()

def make_uniform_data(n, tol=10**4):

    data = np.random.randint(tol, size=n)
    return pd.Series(Counter(data))

def make_zipf_data(n, tol=10**4, parameter=1.5):
    '''Generate data from a windowed zipf distribution but
    If we get a sample above a certain tol, re-sample
    '''
    data = []
    for _ in range(n):

        x = np.random.zipf(parameter)
        while x > tol:
            x = np.random.zipf(parameter)

        data.append(x)

    return pd.Series(Counter(data))


def precision_k(true, re_rank, k):
    '''Fraction of true heavy hitters retrieved

    '''
    
    true_ranks = true.head(k)
    est_ranks = re_rank.head(k)
    
    p_at_k = 0
    for i, _ in true_ranks.items():
        new_rank = np.argwhere(re_rank.index == i)[0][0]
        logger.debug("New Loc of {} is {}".format(i, 
                                                  new_rank))
        if new_rank < k:
            p_at_k  += 1
            
    return p_at_k/k

def base_experiment(histogram, epsilon, delta):
    '''Vanilla Bin Mechanism'''
    
    n_b = int((100/epsilon**2)*(np.log(2/delta)))
    logger.debug("\tEpsilon: {} Delta: {} | Coins: {}".format(epsilon, delta, n_b))
    
    sorted_hist = histogram.sort_values(ascending=False)
    for key, value in sorted_hist.items():
        sorted_hist[key] = sorted_hist[key] + np.random.binomial(n_b, 1/2) - int(n_b/2)
        
    return sorted_hist.sort_values(ascending=False)

def our_experiment(histogram, epsilon, delta, repeats):
    '''Repeated Verfiable Bin Mechanism'''
    
    n_b = int((100/epsilon**2)*(np.log(2/delta)))
    logger.debug("\tEpsilon: {} Delta: {} | Coins: {}".format(epsilon, delta, n_b))
    
    sorted_hist = histogram.sort_values(ascending=False)
    for key, value in sorted_hist.items():
        for _ in range(repeats):
            sorted_hist[key] = sorted_hist[key] + np.random.binomial(n_b, 1/2) - int(n_b/2)
        
    return sorted_hist.sort_values(ascending=False)



